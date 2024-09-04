use image::{DynamicImage, GenericImageView, Luma, Pixel};

use crate::{Args, Vector2, Vector4};

pub fn get_edges(image: DynamicImage, args: &Args) -> Vec<Vector2> {
    let (w, h) = image.dimensions();
    let mut buff: image::ImageBuffer<Luma<u8>, Vec<<Luma<u8> as Pixel>::Subpixel>> = image::ImageBuffer::new(w, h);

    image.pixels().for_each(|(x, y, pixel)| {
        buff.put_pixel(x, y, {
            let c00 = Vector4::from_color(pixel);
            let c01 = Vector4::from_color(image.get_pixel(x, (y+1).clamp(0, h-1)));
            let c10 = Vector4::from_color(image.get_pixel((x+1).clamp(0, w-1), y));
            let dy = c01 - c00;
            let dx = c10 - c00;
            let grad_squ_mag = dy*dy + dx*dx;
            let grad_mag = grad_squ_mag.map(f64::sqrt);
            let is_edge = f64::sqrt(Vector4::dot(grad_mag, grad_mag)) > args.edge_threshold;
            Luma::from(
                [if is_edge {u8::MAX} else {0}]
            )
        });
    });

    if let Some(s) = &args.debug_output_image {
        eprintln!("      saving image");
        buff.save(s).expect("file failed to save");
    }


    buff.enumerate_pixels()
        .filter(|(_, _, px)| {
            px.channels()[0] > 0
        }).map(|(x, y, _)| {
            Vector2 {x: x as f64, y: (h-y) as f64}
        }).collect()
}

pub fn construct_contour(mut edge_points: Vec<Vector2>) -> Vec<Vector2> {
    let mut contour = vec![];
    let mut head: Vector2;
    contour.push(edge_points.pop().unwrap());
    head = contour[0];
    while !edge_points.is_empty() {
        let i = edge_points.iter()
            .enumerate()
            .min_by(|(_, u), (_, v)| {
                let du = head - **u;
                let dv = head - **v;
                Vector2::dot(du, du).total_cmp(&Vector2::dot(dv, dv))
            }).map(|(i, _)| {i});
        head = edge_points.remove(i.unwrap());
        contour.push(head);
    }
    contour
}
