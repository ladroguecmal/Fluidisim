//! Rasterisation locale du banc : maillage projeté, lumière fixe, aucune dépendance.
use std::{fs::File,io::{Write,BufWriter}};
pub const WIDTH:usize=1200;pub const HEIGHT:usize=650;
fn sub(a:[f32;3],b:[f32;3])->[f32;3]{[a[0]-b[0],a[1]-b[1],a[2]-b[2]]}
fn cross(a:[f32;3],b:[f32;3])->[f32;3]{[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]}
fn dot(a:[f32;3],b:[f32;3])->f32{a[0]*b[0]+a[1]*b[1]+a[2]*b[2]}
fn normalize(a:[f32;3])->[f32;3]{let l=dot(a,a).sqrt();a.map(|v|v/l)}
fn project(p:[f32;3],panel:usize)->[f32;3]{
    [panel as f32*600.+300.+43.*(0.8*p[0]-0.6*p[1]),
    350.+43.*(0.36*p[0]+0.48*p[1]-0.8*p[2]),0.48*p[0]+0.64*p[1]+0.6*p[2]]
}
fn triangle(rgb:&mut [u8],depth:&mut [f32],p:[[f32;3];3],color:[u8;3]) {
    let edge=|a:[f32;3],b:[f32;3],x:f32,y:f32|(b[0]-a[0])*(y-a[1])-(b[1]-a[1])*(x-a[0]);
    let area=edge(p[0],p[1],p[2][0],p[2][1]);if area.abs()<1e-8{return;}
    let xmin=p.iter().map(|p|p[0]).fold(f32::INFINITY,f32::min).floor().max(0.) as usize;
    let xmax=p.iter().map(|p|p[0]).fold(f32::NEG_INFINITY,f32::max).ceil().min((WIDTH-1) as f32) as usize;
    let ymin=p.iter().map(|p|p[1]).fold(f32::INFINITY,f32::min).floor().max(0.) as usize;
    let ymax=p.iter().map(|p|p[1]).fold(f32::NEG_INFINITY,f32::max).ceil().min((HEIGHT-1) as f32) as usize;
    for y in ymin..=ymax {for x in xmin..=xmax {
        let a=edge(p[1],p[2],x as f32+0.5,y as f32+0.5)/area;
        let b=edge(p[2],p[0],x as f32+0.5,y as f32+0.5)/area;let c=1.-a-b;
        if a<0. || b<0. || c<0. {continue;}
        let z=a*p[0][2]+b*p[1][2]+c*p[2][2];let q=y*WIDTH+x;
        if z>depth[q] {depth[q]=z;rgb[q*3..q*3+3].copy_from_slice(&color);}
    }}
}
fn line(rgb:&mut[u8],a:[f32;3],b:[f32;3],color:[u8;3]) {
    let n=((a[0]-b[0]).abs().max((a[1]-b[1]).abs())).ceil() as usize;
    for i in 0..=n {let t=i as f32/n.max(1) as f32;let x=(a[0]+t*(b[0]-a[0])).round() as i32;let y=(a[1]+t*(b[1]-a[1])).round() as i32;
        if x>=0 && y>=0 && x<WIDTH as i32 && y<HEIGHT as i32 {let q=(y as usize*WIDTH+x as usize)*3;rgb[q..q+3].copy_from_slice(&color);}}
}
pub fn render(path:&str,nx:usize,ny:usize,dx:f32,total:&[f32],difference:&[f32])->std::io::Result<u64> {
    let mut rgb=vec![0u8;WIDTH*HEIGHT*3];let mut depth=vec![f32::NEG_INFINITY;WIDTH*HEIGHT];
    for y in 0..HEIGHT {for x in 0..WIDTH {let q=(y*WIDTH+x)*3;let v=(7.+9.*y as f32/HEIGHT as f32) as u8;rgb[q..q+3].copy_from_slice(&[v,v+7,v+14]);}}
    let (lx,ly)=(nx as f32*dx,ny as f32*dx);
    for panel in 0..2 {
        let fields=if panel==0 {total} else {difference};let gain=if panel==0 {1.} else {4.};
        // Grille métrique au sol, habillage explicite du banc.
        for i in 0..=lx as usize {line(&mut rgb,project([i as f32-lx/2.,-ly/2.,-0.5],panel),project([i as f32-lx/2.,ly/2.,-0.5],panel),[33,50,66]);}
        for j in 0..=ly as usize {line(&mut rgb,project([-lx/2.,j as f32-ly/2.,-0.5],panel),project([lx/2.,j as f32-ly/2.,-0.5],panel),[33,50,66]);}
        let point=|i:usize,j:usize|[(i as f32+0.5)*dx-lx/2.,(j as f32+0.5)*dx-ly/2.,fields[j*nx+i]*gain];
        for j in 0..ny-1 {for i in 0..nx-1 {
            for ids in [[(i,j),(i+1,j),(i+1,j+1)],[(i,j),(i+1,j+1),(i,j+1)]] {
                let world=ids.map(|(i,j)|point(i,j));
                let normal=normalize(cross(sub(world[1],world[0]),sub(world[2],world[0])));
                let light=0.35+0.65*dot(normal,normalize([-0.5,-0.4,1.])).max(0.);
                let h=ids.iter().map(|(i,j)|fields[j*nx+i]).sum::<f32>()/3.;
                let base=if panel==0 {[34.,150.,183.]}
                    else {let f=(h/0.10).clamp(-1.,1.);if f>=0. {[110.+120.*f,177.-20.*f,184.-110.*f]} else {[110.+50.*f,177.+75.*f,184.+20.*f]}};
                let color=base.map(|v|(v*light).clamp(0.,255.) as u8);
                triangle(&mut rgb,&mut depth,world.map(|p|project(p,panel)),color);
            }
        }}
    }
    let mut hash=0xcbf29ce484222325u64;
    let header=format!("P6\n{WIDTH} {HEIGHT}\n255\n");
    for b in header.bytes().chain(rgb.iter().copied()){hash^=b as u64;hash=hash.wrapping_mul(0x100000001b3);}
    let mut f=BufWriter::new(File::create(path)?);f.write_all(header.as_bytes())?;f.write_all(&rgb)?;Ok(hash)
}
