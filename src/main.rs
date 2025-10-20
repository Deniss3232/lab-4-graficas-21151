use raylib::prelude::*;

/// Convierte NDC (-1..1) a píxeles
fn ndc_to_screen(ndc_x: f32, ndc_y: f32, w: i32, h: i32) -> Vector2 {
    let sx = (ndc_x + 1.0) * 0.5 * w as f32;
    let sy = (1.0 - ndc_y) * 0.5 * h as f32; // flip Y
    Vector2::new(sx, sy)
}

fn main() {
    const W: i32 = 900;
    const H: i32 = 900;

    let (mut rl, th) = raylib::init()
        .size(W, H)
        .title("Rust Graphics - Nave espacial rojo y dorado")
        .build();

    // Paleta
    let bg     = Color::new(18, 18, 26, 255);
    let red    = Color::new(195, 38, 44, 255);
    let red_d  = Color::new(110, 18, 24, 255);
    let gold   = Color::new(230, 182, 62, 255);
    let gold_l = Color::new(255, 212, 96, 255);
    let jet    = Color::new(255, 90, 26, 255);
    let line   = Color::new(70, 22, 16, 255);

    // Rectángulos (top-left NDC, w/h en tamaño NDC)
    let rects: &[(f32, f32, f32, f32, Color)] = &[
        (-0.36,  0.02, 0.72, 0.04, gold),   // lomo
        (-0.12,  0.18, 0.24, 0.06, red_d),  // base cabina
        (-0.04,  0.08, 0.08,  0.14, red),   // deriva
        (-0.78, -0.22, 0.22, 0.14, gold_l), // motor izq (caja)
        ( 0.56, -0.22, 0.22, 0.14, gold_l), // motor der (caja)
    ];

    // Barras (jets)
    let bars: &[(f32, f32, f32, f32, Color)] = &[
        (-0.90, -0.20, 0.12, 0.10, jet),
        ( 0.78, -0.20, 0.12, 0.10, jet),
    ];

    // Triángulos 
    let tris: &[(f32,f32, f32,f32, f32,f32, Color)] = &[
        // Morro
        (-0.10, 0.32,  0.10, 0.32,  0.00, 0.60, gold_l),
        (-0.08, 0.32,  0.08, 0.32,  0.00, 0.48, red_d),


        // Ala izquierda (triángulo grande rojo + sombra interior)
        (-0.62,-0.10, -1.06,-0.44, -0.14,-0.44, red),
        (-0.62,-0.10, -0.30,-0.27, -0.14,-0.44, red_d),
        // Ala derecha = espejo exacto (x -> -x)
        ( 0.62,-0.10,  1.06,-0.44,  0.14,-0.44, red),
        ( 0.62,-0.10,  0.30,-0.27,  0.14,-0.44, red_d),

        // Fuselaje en rombo (encima de las alas)
        (-0.62,-0.10,  0.62,-0.10,  0.36, 0.26, red),
        (-0.62,-0.10,  0.36, 0.26, -0.36, 0.26, red),
        (-0.62,-0.10, -0.12, 0.08, -0.36, 0.26, red_d),
        ( 0.62,-0.10,  0.12, 0.08,  0.36, 0.26, red_d),

        // Canards
        (-0.30, 0.22, -0.16, 0.22, -0.24, 0.30, gold),
        ( 0.30, 0.22,  0.16, 0.22,  0.24, 0.30, gold),

        // Campanas de motor
        (-0.78,-0.22, -0.66,-0.22, -0.72,-0.34, gold),
        ( 0.56,-0.22,  0.68,-0.22,  0.62,-0.34, gold),

        // Decoración interna en ambas alas (espejo también)
        (-0.90,-0.36, -0.74,-0.36, -0.82,-0.22, gold_l),
        (-0.74,-0.36, -0.58,-0.36, -0.66,-0.24, gold),
        ( 0.90,-0.36,  0.74,-0.36,  0.82,-0.22, gold_l),
        ( 0.74,-0.36,  0.58,-0.36,  0.66,-0.24, gold),
    ];

    // Cabina (vidrio) + reflejo
    let cockpit_rect   = (-0.10_f32, 0.18_f32, 0.20_f32, 0.06_f32, Color::new(255, 168, 118, 255));
    let cockpit_reflex = (-0.10_f32, 0.24_f32, -0.02_f32, 0.24_f32, -0.10_f32, 0.18_f32,
                          Color::new(255, 210, 170, 255));

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&th);
        d.clear_background(bg);

        // Rectángulos
        for &(x, y, w, h, c) in rects {
            let p0 = ndc_to_screen(x, y, W, H);
            let r  = Rectangle::new(p0.x, p0.y, w * 0.5 * W as f32, h * 0.5 * H as f32);
            d.draw_rectangle_rec(r, c);
        }

        // Cabina
        {
            let (x,y,w,h,c) = cockpit_rect;
            let p0 = ndc_to_screen(x, y, W, H);
            d.draw_rectangle_rec(Rectangle::new(p0.x, p0.y, w * 0.5 * W as f32, h * 0.5 * H as f32), c);
        }

        // Jets
        for &(x, y, w, h, c) in bars {
            let p0 = ndc_to_screen(x, y, W, H);
            d.draw_rectangle_rec(Rectangle::new(p0.x, p0.y, w * 0.5 * W as f32, h * 0.5 * H as f32), c);
        }

        // Triángulos
        for &(x1,y1, x2,y2, x3,y3, c) in tris {
            let a = ndc_to_screen(x1, y1, W, H);
            let b = ndc_to_screen(x2, y2, W, H);
            let c3= ndc_to_screen(x3, y3, W, H);
            d.draw_triangle(a, b, c3, c);
        }

        // Reflejo de cabina
        {
            let (x1,y1,x2,y2,x3,y3,c) = cockpit_reflex;
            let a = ndc_to_screen(x1, y1, W, H);
            let b = ndc_to_screen(x2, y2, W, H);
            let c3= ndc_to_screen(x3, y3, W, H);
            d.draw_triangle(a, b, c3, c);
        }

        // Bordes/wireframe 
        for &(x, y, w, h, _) in rects.iter().chain(&[cockpit_rect]) {
            let p0 = ndc_to_screen(x, y, W, H);
            let wpx = w * 0.5 * W as f32;
            let hpx = h * 0.5 * H as f32;
            d.draw_rectangle_lines(p0.x as i32, p0.y as i32, wpx as i32, hpx as i32, line);
        }
        for &(x1,y1, x2,y2, x3,y3, _) in tris.iter().chain(std::iter::once(&cockpit_reflex)) {
            let a = ndc_to_screen(x1, y1, W, H);
            let b = ndc_to_screen(x2, y2, W, H);
            let c3= ndc_to_screen(x3, y3, W, H);
            d.draw_line_ex(a, b, 2.0, line);
            d.draw_line_ex(b, c3, 2.0, line);
            d.draw_line_ex(c3, a, 2.0, line);
        }

        d.draw_text("Nave roja y dorada", 18, 18, 22, Color::GOLD);

        if d.is_key_pressed(KeyboardKey::KEY_S) {
            d.take_screenshot(&th, "captura.png");
        }
    }
}
