// S211 : image cosmétique seulement. B et W proviennent du cœur.
// S212 : sillage = coefficients [A, B, kx, ky] rebasés à la caméra par le cœur.
// S234 : LOD spatial de couche — le sillage est évalué sur sa grille locale, au pas que sa borne
// bicubique autorise (lod.rs), puis reconstruit par Hermite bicubique à chaque sommet.
struct Params {
    eye: vec4<f32>, forward: vec4<f32>, right: vec4<f32>, up: vec4<f32>,
    impact: vec4<f32>, info: vec4<f32>, wake_rect: vec4<f32>, wake_info: vec4<f32>,
    lattice: vec4<f32>, // pas (m), nx, ny, 1 = grille / 0 = somme directe par sommet
    spectral: vec4<f32>, // activé, k_max de la recette ; huit bandes ADR-148
}
@group(0) @binding(0) var<uniform> p: Params;
@group(0) @binding(1) var<storage, read> waves: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read> profile: array<vec2<f32>>;
@group(0) @binding(3) var<storage, read> wake: array<vec4<f32>>;
@group(1) @binding(0) var<storage, read> probes: array<vec4<f32>>;
@group(1) @binding(1) var<storage, read_write> results: array<vec4<f32>>;
// S235 : un impact par ligne — centre relatif à la caméra, actif (1/0). `p.impact.x` : longueur
// d'un profil ; `p.info.w` : nombre d'impacts.
@group(0) @binding(4) var<storage, read> impacts: array<vec4<f32>>;
@group(2) @binding(0) var<storage, read> lattice: array<vec4<f32>>;
@group(3) @binding(0) var<storage, read_write> lattice_out: array<vec4<f32>>;

fn spectral_weight(k: f32, h: f32) -> f32 {
    let t = clamp(2.0*k*h/3.141592653589793 - 1.0, 0.0, 1.0);
    return 1.0 - t*t*(3.0 - 2.0*t);
}
fn band_upper(b: u32) -> f32 { return p.spectral.y / f32(1u << b); }
fn spectral_band(k: f32) -> u32 {
    var b = 0u;
    var upper = p.spectral.y;
    while (b < 7u && k <= upper*0.5) { b++; upper *= 0.5; }
    return b;
}

// Somme modale du sillage : hauteur, deux pentes et dérivée croisée. Sans elle, la grille ne
// reçoit pas la reconstruction bicubique dont lod.rs publie la borne.
fn wake_direct(q: vec2<f32>) -> vec4<f32> {
    var v = vec4<f32>(0.0);
    for (var i = 0u; i < u32(p.wake_info.x); i++) {
        let c = wake[i];
        let phase = dot(c.zw, q);
        let s = sin(phase);
        let co = cos(phase);
        let e = c.x*co - c.y*s;
        let d = -(c.x*s + c.y*co);
        v += vec4<f32>(e, d*c.zw, -e*c.z*c.w);
    }
    return v;
}
@compute @workgroup_size(8, 8)
fn bake(@builtin(global_invocation_id) id: vec3<u32>) {
    let nx = u32(p.lattice.y); let ny = u32(p.lattice.z);
    if (id.x >= nx || id.y >= ny) { return; }
    let q = p.wake_rect.xy + vec2<f32>(f32(id.x), f32(id.y))*p.lattice.x;
    let index = id.y*nx + id.x;
    if (p.spectral.x < 0.5) { lattice_out[index] = wake_direct(q); return; }
    var bands: array<vec4<f32>, 8>;
    var total = vec4<f32>(0.0);
    for (var i = 0u; i < u32(p.wake_info.x); i++) {
        let c = wake[i];
        let phase = dot(c.zw, q);
        let s = sin(phase); let co = cos(phase);
        let e = c.x*co - c.y*s;
        let d = -(c.x*s + c.y*co);
        let v = vec4<f32>(e, d*c.zw, -e*c.z*c.w);
        let b = spectral_band(length(c.zw));
        bands[b] += v;
        total += v;
    }
    lattice_out[index] = total;
    for (var b = 0u; b < 8u; b++) { lattice_out[(b+1u)*nx*ny + index] = bands[b]; }
}
// Mêmes opérations que `lod::hermite`.
fn wake_lattice(q: vec2<f32>, band: u32) -> vec3<f32> {
    let s = p.lattice.x; let nx = u32(p.lattice.y); let ny = u32(p.lattice.z);
    let u = (q - p.wake_rect.xy)/s;
    let i = min(u32(floor(u.x)), nx - 2u);
    let j = min(u32(floor(u.y)), ny - 2u);
    let tx = u.x - f32(i); let ty = u.y - f32(j);
    let tx2 = tx*tx; let tx3 = tx2*tx; let ty2 = ty*ty; let ty3 = ty2*ty;
    let vx = vec2<f32>(2*tx3-3*tx2+1, -2*tx3+3*tx2);
    let dx = vec2<f32>(tx3-2*tx2+tx, tx3-tx2);
    let vx1 = vec2<f32>(6*tx2-6*tx, -6*tx2+6*tx);
    let dx1 = vec2<f32>(3*tx2-4*tx+1, 3*tx2-2*tx);
    let vy = vec2<f32>(2*ty3-3*ty2+1, -2*ty3+3*ty2);
    let dy = vec2<f32>(ty3-2*ty2+ty, ty3-ty2);
    let vy1 = vec2<f32>(6*ty2-6*ty, -6*ty2+6*ty);
    let dy1 = vec2<f32>(3*ty2-4*ty+1, 3*ty2-2*ty);
    var out = vec3<f32>(0.0);
    for (var b = 0u; b < 2u; b++) {
        for (var a = 0u; a < 2u; a++) {
            let c = lattice[band*nx*ny + (j+b)*nx + i + a];
            let f = c.x; let fx = c.y*s; let fy = c.z*s; let fxy = c.w*s*s;
            out.x += vx[a]*vy[b]*f + dx[a]*vy[b]*fx + vx[a]*dy[b]*fy + dx[a]*dy[b]*fxy;
            out.y += vx1[a]*vy[b]*f + dx1[a]*vy[b]*fx + vx1[a]*dy[b]*fy + dx1[a]*dy[b]*fxy;
            out.z += vx[a]*vy1[b]*f + dx[a]*vy1[b]*fx + vx[a]*dy1[b]*fy + dx[a]*dy1[b]*fxy;
        }
    }
    return vec3<f32>(out.x, out.y/s, out.z/s);
}

// Hauteur et deux pentes. Une seule fonction pour sommets et réception compute.
fn water(q: vec2<f32>, spacing: f32) -> vec3<f32> {
    let h = select(0.0, spacing, p.spectral.x > 0.5);
    var v = vec3<f32>(0.0);
    for (var i = 0u; i < u32(p.info.x); i++) {
        let c = waves[i];
        let phase = dot(c.yz, q) + c.w;
        let a = c.x * spectral_weight(length(c.yz), h);
        v += vec3<f32>(a * sin(phase), a * cos(phase) * c.yz);
    }
    for (var m = 0u; m < u32(p.info.w); m++) {
        let c = impacts[m];
        let d = q - c.xy;
        let r = length(d);
        if (c.z < 0.5 || r > p.impact.z) { continue; }
        let base = m*u32(p.impact.x);
        let x = r / p.impact.w;
        let i = u32(x);
        let t = x - f32(i);
        let t2 = t*t;
        let t3 = t2*t;
        let a = profile[base+i]; let b = profile[base+i+1u];
        let m0 = a.y*p.impact.w; let m1 = b.y*p.impact.w;
        let h = (2*t3-3*t2+1)*a.x + (t3-2*t2+t)*m0 + (-2*t3+3*t2)*b.x + (t3-t2)*m1;
        let slope = ((6*t2-6*t)*a.x+(3*t2-4*t+1)*m0+(-6*t2+6*t)*b.x+(3*t2-2*t)*m1)/p.impact.w;
        var gradient = vec2<f32>(0.0);
        if (r > 0.0) { gradient = slope*d/r; }
        v += vec3<f32>(h, gradient);
    }
    // Même emprise que le cœur ; hors emprise le sillage ne contribue pas (couture publiée).
    if (p.wake_info.y > 0.5 && all(q >= p.wake_rect.xy) && all(q <= p.wake_rect.zw)) {
        if (p.lattice.w > 0.5) {
            if (spectral_weight(p.spectral.y, h) == 1.0) {
                v += wake_lattice(q, 0u);
            } else {
                for (var b = 0u; b < 8u; b++) {
                    let a = spectral_weight(band_upper(b), h);
                    if (a > 0.0) { v += a*wake_lattice(q, b+1u); }
                }
            }
        } else {
            for (var i = 0u; i < u32(p.wake_info.x); i++) {
                let c = wake[i];
                var a = 1.0;
                if (h > 0.0) { a = spectral_weight(band_upper(spectral_band(length(c.zw))), h); }
                if (a == 0.0) { continue; }
                let phase = dot(c.zw, q);
                let s = sin(phase);
                let co = cos(phase);
                v += a*vec3<f32>(c.x*co - c.y*s, -(c.x*s + c.y*co)*c.zw);
            }
        }
    }
    return v;
}
@compute @workgroup_size(64)
fn verify(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x < arrayLength(&probes)) {
        let probe = probes[id.x];
        var q = probe.xy; var h = probe.z;
        if (probe.w > 0.5) { q = grid_point(probe.xy); h = grid_spacing(probe.xy, q); }
        results[id.x] = vec4<f32>(water(q, h), h);
    }
}
struct Vertex { @builtin(position) clip: vec4<f32>, @location(0) local: vec3<f32>, @location(1) slope: vec2<f32> }
fn grid_point(index: vec2<f32>) -> vec2<f32> {
    let nx = u32(p.info.y); let ny = u32(p.info.z);
    let x = (index.x/f32(nx-1u)*2-1)*1.18;
    let horizon = clamp(-p.forward.z/(p.up.z*p.forward.w), -0.95, 1.2);
    let y = -1.18 + (horizon-0.003+1.18)*index.y/f32(ny-1u);
    let ray = p.forward.xyz + p.right.xyz*x*p.forward.w*p.right.w + p.up.xyz*y*p.forward.w;
    let distance = min(p.eye.z/max(-ray.z,0.00001),1500.0);
    return ray.xy*distance;
}
fn grid_spacing(index: vec2<f32>, q: vec2<f32>) -> f32 {
    var h = 0.0;
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let neighbor = clamp(index + vec2<f32>(f32(x), f32(y)), vec2<f32>(0.0), p.info.yz-vec2<f32>(1.0));
            h = max(h, length(grid_point(neighbor) - q));
        }
    }
    return h;
}
@vertex fn ocean_vertex(@builtin(vertex_index) id: u32) -> Vertex {
    let nx = u32(p.info.y);
    let index = vec2<f32>(f32(id % nx), f32(id / nx));
    let q = grid_point(index);
    var h = 0.0;
    if (p.spectral.x > 0.5) { h = grid_spacing(index, q); }
    let w = water(q, h);
    let local = vec3<f32>(q,w.x-p.eye.z);
    let depth = max(dot(local,p.forward.xyz),0.01);
    var o: Vertex;
    o.clip = vec4<f32>(dot(local,p.right.xyz)/(p.forward.w*p.right.w),dot(local,p.up.xyz)/p.forward.w,depth-0.1,depth);
    o.local = local; o.slope = w.yz;
    return o;
}
fn sky(ray: vec3<f32>) -> vec3<f32> {
    let sun = normalize(vec3<f32>(-0.4,0.3,0.8));
    return mix(vec3<f32>(0.66,0.78,0.84),vec3<f32>(0.18,0.39,0.65),clamp(ray.z,0.0,1.0))
        + vec3<f32>(1.0,0.84,0.6)*pow(max(dot(ray,sun),0.0),512.0);
}
@fragment fn ocean_fragment(v: Vertex) -> @location(0) vec4<f32> {
    let n = normalize(vec3<f32>(-v.slope,1.0));
    let ray = normalize(v.local);
    let fresnel = 0.02+0.98*pow(1.0-max(dot(-ray,n),0.0),5.0);
    let sun = normalize(vec3<f32>(-0.4,0.3,0.8));
    let reflection = reflect(ray,n);
    let glint = pow(max(dot(reflection,sun),0.0),180.0);
    let base = vec3<f32>(0.012,0.105,0.13)*(0.65+0.35*max(dot(n,sun),0.0));
    let color = mix(base,sky(reflection),fresnel)+vec3<f32>(1.0,0.9,0.7)*glint*0.65;
    return vec4<f32>(mix(color,vec3<f32>(0.66,0.78,0.84),1-exp(-length(v.local)/500.0)),1.0);
}
struct SkyVertex { @builtin(position) clip: vec4<f32>, @location(0) uv: vec2<f32> }
@vertex fn sky_vertex(@builtin(vertex_index) id: u32) -> SkyVertex {
    let uv = vec2<f32>(f32((id<<1u)&2u),f32(id&2u))*2-1;
    var o: SkyVertex; o.clip = vec4<f32>(uv,0.99999,1.0); o.uv=uv; return o;
}
@fragment fn sky_fragment(v: SkyVertex) -> @location(0) vec4<f32> {
    let ray = normalize(p.forward.xyz+p.right.xyz*v.uv.x*p.forward.w*p.right.w+p.up.xyz*v.uv.y*p.forward.w);
    return vec4<f32>(sky(ray),1.0);
}
