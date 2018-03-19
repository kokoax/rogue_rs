extern crate ncurses;
extern crate rand;

use std::vec::Vec;
use ncurses::*;
use rand::Rng;

const MAP_WIDTH:  usize = 64;
const MAP_HEIGHT: usize = 32;

#[derive(Copy)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    fn new() -> Point2D {
        Point2D {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl std::clone::Clone for Point2D {
    fn clone(&self) -> Point2D { *self }
}

#[derive(Copy)]
struct RoomData {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl RoomData {
    fn new() -> RoomData {
        RoomData {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
        }
    }
}

impl std::clone::Clone for RoomData {
    fn clone(&self) -> RoomData { *self }
}

struct RoomGenerator {
}

impl RoomGenerator {
    fn generate_point() -> Vec<Point2D> {
        // TODO: マップに対して何個のinstanceがあれば網羅的かを調べる
        // 現在、map全ての点の数の1/20個のinstanceを生成
        const point_num: usize = (MAP_HEIGHT*MAP_WIDTH)/90;
        let mut points = vec![Point2D::new(); point_num as usize];
        for i in 0..point_num {
            points[i].x = rand::thread_rng().gen_range(0, MAP_WIDTH)  as f64;
            points[i].y = rand::thread_rng().gen_range(0, MAP_HEIGHT) as f64;
        }
        return points.to_vec();
    }

    fn dist(p: Point2D, q: Point2D) -> f64 {
        ((p.x-q.x).powf(2.0)+(p.y-q.y).powf(2.0)).sqrt()
    }

    fn generate_centroid(k: usize) -> Vec<Point2D> {
        let mut centroids = vec![Point2D::new(); k as usize];
        for i in 0..k {
            centroids[i].x = rand::thread_rng().gen_range(0, MAP_WIDTH) as f64;
            centroids[i].y = rand::thread_rng().gen_range(0, MAP_HEIGHT) as f64;
        }
        return centroids.to_vec();
    }

    fn generate_cluster(k: usize, len: usize) -> Vec<u32> {
        let mut cluster: Vec<u32> = vec![0; len];
        for i in 0..len {
            cluster[i] = rand::thread_rng().gen_range(0, k) as u32;
        }
        return cluster;
    }

    fn sum(vector: &Vec<f64>, len: usize) -> f64 {
        let mut sum = 0 as f64;
        for i in 0..len {
            sum += vector[i];
        }
        sum
    }

    fn mean(vector: &Vec<f64>, len: usize) -> f64 {
        // let len = vector.len();
        RoomGenerator::sum(vector, len) / (len as f64)
    }

    fn init_tmp(tmp: &mut Vec<f64>) {
        for i in 0..(*tmp).len() {
            (*tmp)[i] = 0.0;
        }
    }

    fn get_centroid(k: usize, cluster: &Vec<u32>, feature: &Vec<Point2D>) ->  Vec<Point2D> {
        let mut tmp_x: Vec<f64> = vec![0.0; feature.len() as usize];
        let mut tmp_y: Vec<f64> = vec![0.0; feature.len() as usize];
        let mut centroids = vec![Point2D::new(); k];
        let mut tmp_i = 0;
        for class in 0..k {
            tmp_i = 0;
            RoomGenerator::init_tmp(&mut tmp_x);
            RoomGenerator::init_tmp(&mut tmp_y);
            for i in 0..cluster.len() {
                if class as u32 == cluster[i] {
                    tmp_x[tmp_i] = feature[i].x;
                    tmp_y[tmp_i] = feature[i].y;
                    tmp_i += 1;
                }
            }
            centroids[class as usize] =
                Point2D{
                    x: RoomGenerator::mean(&tmp_x, tmp_i),
                    y: RoomGenerator::mean(&tmp_y, tmp_i)
                };
        }
        return centroids;
    }

    fn get_cluster(centroids: &Vec<Point2D>, feature: &Vec<Point2D>) -> Vec<u32> {
        let mut cluster: Vec<u32> = vec![0; feature.len() as usize];
        for i in 0..feature.len() {
            let mut min   = RoomGenerator::dist(feature[i], centroids[0]);
            let mut min_i = 0;
            for j in 1..centroids.len() {
                let dist_tmp = RoomGenerator::dist(feature[i], centroids[j]);
                if dist_tmp < min {
                    min   = dist_tmp;
                    min_i = j as u32;
                }
            }
            cluster[i] = min_i;
        }
        return cluster;
    }

    fn eq_with_nan_eq(a: Point2D, b: Point2D) -> bool {
        // (a.is_nan() && b.is_nan()) || (a.x == b.x && a.y == b.y)
        a.x == b.x && a.y == b.y
    }

    fn compare_point(p1: Point2D, p2: Point2D) -> bool {
        p1.x == p2.x && p1.y == p2.y
    }

    fn compare_vec(va: &Vec<Point2D>, vb: &Vec<Point2D>) -> bool {
        if (*va).len() == (*vb).len() {
            for i in 0..(*va).len() {
                if !RoomGenerator::compare_point((*va)[i],(*vb)[i]) {
                    return false;
                }
            }
        } else {
            return false;
        }
        return true;
    }

    fn k_means(k: usize, feature: &Vec<Point2D>) -> Vec<u32> {
        // let mut centroids = RoomGenerator::generate_centroid(k);
        let mut cluster: Vec<u32> = RoomGenerator::generate_cluster(k, (*feature).len());
        let mut centroids = RoomGenerator::get_centroid(k, &cluster, feature);
        let mut old_centroids = vec![Point2D::new(); k];
        while !RoomGenerator::compare_vec(&old_centroids, &centroids) {
            RoomGenerator::view_feature(&cluster, feature, &centroids);
            old_centroids = centroids.clone(); // TODO: to pointer
            cluster   = RoomGenerator::get_cluster(&centroids, feature);
            centroids = RoomGenerator::get_centroid(k, &cluster, feature);
        }
        return cluster;
    }

    fn max(x: f64, y: f64) -> f64 {
        if x < y {
            return y;
        } else {
            return x;
        }
    }

    fn min(x: f64 , y: f64) -> f64 {
        if x < y {
            return x;
        } else {
            return y;
        }
    }

    fn view_feature(cluster: &Vec<u32>, feature: &Vec<Point2D>, centroids: &Vec<Point2D>) {
        for i in 1..(*feature).len(){
            mv((*feature)[i].y as i32, (*feature)[i].x as i32);
            printw(&((*cluster)[i].to_string()));
        }
        for i in 0..(*centroids).len() {
            mv((*centroids)[i].y as i32, (*centroids)[i].x as i32);
            printw(&format!("{}.", i));
        }
        getch();
        clear();
    }

    fn get_room_info(k: usize, cluster: Vec<u32>, feature: &Vec<Point2D>) -> Vec<RoomData> {
        let mut room_data = vec![RoomData::new(); k];
        let mut room_tmp = RoomData::new();
        for i in 0..k {
            room_tmp.x = MAP_WIDTH  as f64;
            room_tmp.y = MAP_HEIGHT as f64;
            room_tmp.w = 0 as f64;
            room_tmp.h = 0 as f64;
            for j in 1..cluster.len() {
                if i == cluster[j] as usize {
                    room_tmp.x = RoomGenerator::min(room_tmp.x, (*feature)[j].x);
                    room_tmp.y = RoomGenerator::min(room_tmp.y, (*feature)[j].y);
                    room_tmp.w = RoomGenerator::max(room_tmp.w, (*feature)[j].x);
                    room_tmp.h = RoomGenerator::max(room_tmp.h, (*feature)[j].y);
                }
            }
            room_tmp.w = room_tmp.w - room_tmp.x;
            room_tmp.h = room_tmp.h - room_tmp.y;
            room_data[i] = room_tmp.clone();
        }
        for i in 0..room_data.len() {
            mv(i as i32, 0);
            printw(&format!("x:{} y:{} w:{} h:{}", room_data[i].x, room_data[i].y, room_data[i].w, room_data[i].h));
        }
        getch();
        return room_data;
    }

    fn clustering() -> Vec<RoomData> {
        let room_num: usize = rand::thread_rng().gen_range(3, 9-1); // 3 ~ 9個の部屋にする(クラスタ数に当たる)
        let points: Vec<Point2D>   = RoomGenerator::generate_point();
        let cluster  = RoomGenerator::k_means(room_num, &points);
        return RoomGenerator::get_room_info(room_num, cluster, &points);
    }
}

struct MapController {
    x: u32,
    y: u32,
    map: Vec<Vec<char>>,
}

impl MapController {
    fn new() -> MapController {
        MapController {
            x: 0,
            y: 0,
            map: MapController::generate_map(),
        }
    }

    fn remap(&mut self) -> MapController {
        self.map = MapController::generate_map();
        MapController::new()
    }


    fn init_map() -> Vec<Vec<char>> {
        [['#';MAP_WIDTH];MAP_HEIGHT]
            .into_iter()
                .map(|item| item.to_vec())
                .collect()
    }

    fn generate_room(map: Vec<Vec<char>>) -> Vec<RoomData> {
        let rooms = RoomGenerator::clustering();
        for i in 0..(rooms.len() as usize) {
            mv(i as i32, 0);
            printw(&(rooms[i].x.to_string()));
            printw(" ");
            printw(&(rooms[i].y.to_string()));
            printw(" ");
            printw(&(rooms[i].w.to_string()));
            printw(" ");
            printw(&(rooms[i].h.to_string()));
            printw(" ");
        }
        return rooms;
    }

    fn put_room(room: Vec<RoomData>, map: &mut Vec<Vec<char>>) -> Vec<Vec<char>>{
        let mut i = 0;
        for it in room {
            for ly in 0..(it.h as u32) {
                for lx in 0..(it.w as u32) {
                    // (*map)[((it.y as u32) + ly) as usize][((it.x as u32)+lx) as usize] = (97+i as u8) as char;
                    (*map)[((it.y as u32) + ly) as usize][((it.x as u32)+lx) as usize] = '.';
                }
            }
            i += 1;
        }
        return (*map).clone();
    }

    fn generate_map() -> Vec<Vec<char>> {
        let mut map_tmp = MapController::init_map();
        let rooms = MapController::generate_room(map_tmp.clone());
        return MapController::put_room(rooms, &mut map_tmp);
    }

    fn view(self) {
        clear();
        for i in self.y..(MAP_HEIGHT as u32)+self.y {
            for j in self.x..(MAP_WIDTH as u32)+self.x {
                self.map[i as usize][j as usize] == '1';
                printw(&self.map[i as usize][j as usize].to_string());
            }
            mv((i-self.y) as i32, 0);
        }
    }
}

fn test() {
    mv(0 as i32, 0 as i32);
    // mean function test
    let mut vector: Vec<f64> = vec![0.0; 10 as usize];
    vector[0] = 1.0;
    vector[1] = 2.0;
    vector[2] = 3.0;
    vector[3] = 4.0;
    vector[4] = 5.0;
    vector[5] = 6.0;
    vector[6] = 7.0;
    vector[7] = 8.0;
    vector[8] = 9.0;
    vector[9] = 10.0;
    printw(&(RoomGenerator::mean(&vector, 10).to_string())); // == 5.5

    // distance function test
    mv(1 as i32, 0 as i32);
    let mut p: Point2D = Point2D::new();
    let mut q: Point2D = Point2D::new();

    p.x = 0.0;
    p.y = 0.0;
    q.x = 3.0;
    q.y = 4.0;
    printw(&(RoomGenerator::dist(p,q).to_string())); // == 5.0
}

/* setup ncurses */
fn init_ncurses() {
    initscr();
}

fn main() {
    init_ncurses();

    // test();

    let map = MapController::new();
    map.view();

    getch();

    endwin();
}
