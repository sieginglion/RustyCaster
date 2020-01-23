use std::fs;
use std::time;

fn load_object(name: &str) -> Vec<[[f32; 3]; 3]> {
    let file = fs::read_to_string(name).unwrap();
    let mut vertices = Vec::new();
    let mut object = Vec::new();
    for line in file.split("\n") {
        if &line[0..1] == "v" {
            let vertex: Vec<f32> = line[2..].split(" ").map(|x| x.parse::<f32>().unwrap()).collect();
            vertices.push([vertex[1], vertex[0], vertex[2]]);
        }
        else {
            let indices: Vec<usize> = line[2..].split(" ").map(|x| x.parse::<usize>().unwrap() - 1).collect();
            object.push([vertices[indices[0]], vertices[indices[1]], vertices[indices[2]]]);
        }
    }
    object.push([[100., 0., 0.], [-100., -100., 0.], [-100., 100., 0.]]);
    return object;
}

fn subtract(x: [f32; 3], y: [f32; 3]) -> [f32; 3] {
    return [x[0] - y[0], x[1] - y[1], x[2] - y[2]]
}

fn cross(x: [f32; 3], y: [f32; 3]) -> [f32; 3] {
    return [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]]
}

fn dot(x: [f32; 3], y: [f32; 3]) -> f32 {
    return x[0] * y[0] + x[1] * y[1] + x[2] * y[2]
}

fn intersect(O: [f32; 3], D: [f32; 3], V: &[[f32; 3]; 3]) -> f32 {
    let E_1 = subtract(V[1], V[0]);
    let E_2 = subtract(V[2], V[0]);
    let P = cross(D, E_2);
    let d = dot(E_1, P);
    if d.abs() < 0.000001 {
        return 0.
    }
    let T = subtract(O, V[0]);
    let u = dot(P, T) / d;
    if u < 0. || u > 1. {
        return 0.
    }
    let Q = cross(T, E_1);
    let v = dot(D, Q) / d;
    if v < 0. || u + v > 1. {
        return 0.
    }
    return dot(E_2, Q) / d
}

fn cast(O: &Vec<[[f32; 3]; 3]>, R_0: [f32; 3], D: [f32; 3]) -> f32 {
    for V in O {
        let t = intersect(R_0, D, V);
        if t > 0. {
            return t
        }
    }
    return 1000.
}

fn save_to_image(matrix: Vec<Vec<u8>>) {
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    let mut buffer = image::ImageBuffer::new(n_cols as u32, n_rows as u32);
    for i in 0..n_rows {
        for j in 0..n_cols {
            buffer.put_pixel(j as u32, i as u32, image::Luma([matrix[i][j]]));
        }
    }
    buffer.save("image.png").unwrap();
}

fn square(x: [f32; 3]) -> [f32; 3] {
    return [x[0] * x[0], x[1] * x[1], x[2] * x[2]];
}

fn sum(x: [f32; 3]) -> f32 {
    return x[0] + x[1] + x[2];
}

fn clip(x: f32) -> f32 {
    if x < 0. {
        return 0.;
    }
    return x;
}

fn main() {
    let FOV: f32 = 60. * 0.0174;
    let width = 640;
    let height = 360;
    let mut object = load_object("teapot.obj");
    let camera = [8., 0., 1.5];
    let decay: f32 = 20.;

    let pixel = (FOV / 2.).tan() * 2. / (width as f32);
    let offset_x = - pixel * (width as f32) / 2.;
    let offset_y = pixel * (height as f32) / 2.;
    let mut screen = vec![vec![0u8; width]; height];
    
    let timer = time::Instant::now();
    object.sort_by_key(|x| sum(square(subtract(x[0], camera))) as i32);
    for (i, x) in screen.iter_mut().enumerate() {
        for (j, y) in x.iter_mut().enumerate() {
            let ray = [-1., offset_x + pixel * (j as f32), offset_y - pixel * (i as f32)];
            let t = cast(&object, camera, ray);
            *y = clip(255. - decay * t) as u8;
        }
    }
    println!("{:?}", timer.elapsed());

    save_to_image(screen);
}