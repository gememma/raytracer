use crate::framebuffer::FrameBuffer;
use std::mem::swap;

pub fn draw_line(f: &mut FrameBuffer, sx: i32, sy: i32, ex: i32, ey: i32) {
    if (sx == ex) && (sy == ey) {
        f.plot_pixel(sx as usize, sy as usize, 1., 1., 1.);
    } else if ((ex - sx) as i32).abs() >= ((ey - sy) as i32).abs() {
        draw_x_line(f, sx, sy, ex, ey);
    } else {
        draw_y_line(f, sx, sy, ex, ey);
    }
}

fn draw_x_line(f: &mut FrameBuffer, mut sx: i32, mut sy: i32, mut ex: i32, mut ey: i32) {
    if sx > ex {
        swap(&mut sx, &mut ex);
        swap(&mut sy, &mut ey);
    }
    let dir = if sy < ey { 1 } else { -1 };
    let mut y = sy;
    let mut x = sx;

    let dy = (ey - sy) * dir;
    let dx = ex - sx;
    let mut fy = dy / 2;

    while x != ex {
        if (y as usize) < f.height() && (x as usize) < f.width() {
            f.plot_pixel(x as usize, y as usize, 1., 1., 1.);
        }
        x += 1;
        fy += dy;
        if fy > dx {
            y += dir;
            fy -= dx;
        }
    }
}

fn draw_y_line(f: &mut FrameBuffer, mut sx: i32, mut sy: i32, mut ex: i32, mut ey: i32) {
    if sy > ey {
        swap(&mut sy, &mut ey);
        swap(&mut sx, &mut ex);
    }
    let dir = if sx < ex { 1 } else { -1 };
    let mut y = sy;
    let mut x = sx;

    let dy = ey - sy;
    let dx = (ex - sx) * dir;
    let mut fx = dx / 2;

    while y != ey {
        if (y as usize) < f.height() && (x as usize) < f.width() {
            f.plot_pixel(x as usize, y as usize, 1., 1., 1.);
        }
        y += 1;
        fx += dx;
        if fx > dy {
            x += dir;
            fx -= dy;
        }
    }
}
