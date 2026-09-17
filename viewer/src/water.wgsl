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
// S256, ADR-155 : queue spectrale de B `[a, kx, ky, phase]`, k croissant ; `p.spectral.z` lignes.
@group(0) @binding(5) var<storage, read> tail: array<vec4<f32>>;
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

// S260, ADR-157 — CWM : déplacement de Lagrange et ∂D (xx, xy, yy) de la bande (poids d'ADR-148),
// pentes et ∂D de la queue (poids d'ADR-155, arrêt au premier poids nul).
struct Cwm { d: vec2<f32>, s: vec2<f32>, g: vec3<f32> }
fn band_cwm(q: vec2<f32>, h: f32) -> Cwm {
    var o: Cwm;
    for (var i = 0u; i < u32(p.info.x); i++) {
        let c = waves[i];
        let k = length(c.yz);
        if (k == 0.0) { continue; }
        let a = c.x*spectral_weight(k, h);
        let phase = dot(c.yz, q) + c.w;
        let u = c.yz/k;
        let cs = cos(phase); let sn = sin(phase);
        o.d += a*cs*u;
        o.s += a*cs*c.yz;
        o.g -= a*k*sn*vec3<f32>(u.x*u.x, u.x*u.y, u.y*u.y);
    }
    return o;
}
fn tail_cwm(q: vec2<f32>, h: f32) -> Cwm {
    var o: Cwm;
    for (var i = 0u; i < u32(p.spectral.z); i++) {
        let c = tail[i];
        let k = length(c.yz);
        let w = spectral_weight(k, h);
        if (w == 0.0) { break; }
        let a = w*c.x;
        let phase = dot(c.yz, q) + c.w;
        let u = c.yz/k;
        let cs = cos(phase); let sn = sin(phase);
        o.s += a*cs*c.yz;
        o.g -= a*k*sn*vec3<f32>(u.x*u.x, u.x*u.y, u.y*u.y);
    }
    return o;
}
// Pente eulérienne J⁻ᵀ·s, J = I + g (symétrique) ; `det` rendu pour le repli.
fn euler_slope(s: vec2<f32>, g: vec3<f32>) -> vec3<f32> {
    let jxx = 1.0 + g.x; let jxy = g.y; let jyy = 1.0 + g.z;
    let det = jxx*jyy - jxy*jxy;
    return vec3<f32>((jyy*s.x - jxy*s.y)/det, (-jxy*s.x + jxx*s.y)/det, det);
}
// S256 — pentes de la queue spectrale en `q`, pour une empreinte `h` (m) ; poids d'ADR-148, arrêt à
// la première composante de poids nul (k croissant). Jamais de hauteur.
fn tail_slope(q: vec2<f32>, h: f32) -> vec2<f32> {
    var s = vec2<f32>(0.0);
    for (var i = 0u; i < u32(p.spectral.z); i++) {
        let c = tail[i];
        let w = spectral_weight(length(c.yz), h);
        if (w == 0.0) { break; }
        s += w*c.x*cos(dot(c.yz, q) + c.w)*c.yz;
    }
    return s;
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
        // S256 : sonde de queue (w = 2) — pentes de queue seules, empreinte imposée.
        // S260 : w = 3 — déplacement de la bande ; w = 4 — pente eulérienne CWM (bande + queue), det.
        if (probe.w > 3.5) {
            let b = band_cwm(probe.xy, probe.z); let t = tail_cwm(probe.xy, probe.z);
            let e = euler_slope(b.s + t.s, b.g + t.g);
            results[id.x] = vec4<f32>(e.z, e.xy, probe.z); return;
        }
        if (probe.w > 2.5) { results[id.x] = vec4<f32>(band_cwm(probe.xy, probe.z).d, 0.0, probe.z); return; }
        if (probe.w > 1.5) { results[id.x] = vec4<f32>(0.0, tail_slope(probe.xy, probe.z), probe.z); return; }
        var q = probe.xy; var h = probe.z;
        if (probe.w > 0.5) { q = grid_point(probe.xy); h = grid_spacing(probe.xy, q); }
        results[id.x] = vec4<f32>(water(q, h), h);
    }
}
struct Vertex { @builtin(position) clip: vec4<f32>, @location(0) local: vec3<f32>, @location(1) slope: vec2<f32>, @location(2) lag: vec2<f32>, @location(3) g: vec3<f32> }
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
    var local = vec3<f32>(q,w.x-p.eye.z);
    var o: Vertex;
    o.lag = q;
    // S260, ADR-157 : sommet déplacé de D_B ; W reste évalué au point de Lagrange.
    if (p.spectral.w > 0.5) {
        let b = band_cwm(q, h);
        local = vec3<f32>(q + b.d, w.x-p.eye.z);
        o.g = b.g;
    }
    let depth = max(dot(local,p.forward.xyz),0.01);
    o.clip = vec4<f32>(dot(local,p.right.xyz)/(p.forward.w*p.right.w),dot(local,p.up.xyz)/p.forward.w,depth-0.1,depth);
    o.local = local; o.slope = w.yz;
    return o;
}
// S261 — HABILLAGE DE BANC, sans physique (REVUE-VISUELLE §4). `p.eye.w` : 0 = brume S211,
// 1 = « ciel clair » d'après la référence A de l'utilisateur (couleurs relevées sur la photo,
// converties en linéaire). Rien de ce bloc n'entre dans une réception physique.
const CLEAR_HORIZON = vec3<f32>(0.694, 0.838, 0.930);
const CLEAR_ZENITH = vec3<f32>(0.015, 0.150, 0.600);
fn hash2(q: vec2<f32>) -> f32 {
    let h = dot(q, vec2<f32>(127.1, 311.7));
    return fract(sin(h)*43758.5453);
}
fn value_noise(q: vec2<f32>) -> f32 {
    let i = floor(q); let f = fract(q);
    let u = f*f*(3.0 - 2.0*f);
    let a = hash2(i); let b = hash2(i + vec2<f32>(1.0, 0.0));
    let c = hash2(i + vec2<f32>(0.0, 1.0)); let d = hash2(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}
fn clouds(ray: vec3<f32>) -> f32 {
    if (ray.z <= 0.0) { return 0.0; }
    // Plan de nuages à 1,2 km ; quatre octaves ; couverture faible, basse sur l'horizon, bords doux.
    // Sous 3° d'élévation, le plan projeté dégénère : les nuages s'effacent dans la brume d'horizon.
    let fade = smoothstep(0.05, 0.12, ray.z)*(1.0 - smoothstep(0.30, 0.55, ray.z));
    if (fade <= 0.0) { return 0.0; }
    let q = ray.xy/ray.z*1.3;
    let n = 0.50*value_noise(q) + 0.25*value_noise(q*2.03 + 17.0) + 0.15*value_noise(q*4.11 + 41.0)
        + 0.10*value_noise(q*8.17 + 83.0);
    return smoothstep(0.52, 0.80, n)*fade;
}
fn sky(ray: vec3<f32>) -> vec3<f32> {
    let sun = normalize(vec3<f32>(-0.4,0.3,0.8));
    if (p.eye.w > 0.5) {
        let t = pow(clamp(ray.z, 0.0, 1.0), 0.35);
        var c = mix(CLEAR_HORIZON, CLEAR_ZENITH, t);
        c = mix(c, vec3<f32>(0.92, 0.93, 0.95), clouds(ray));
        return c + vec3<f32>(1.0,0.95,0.85)*(pow(max(dot(ray,sun),0.0),1024.0)*4.0 + pow(max(dot(ray,sun),0.0),32.0)*0.15);
    }
    return mix(vec3<f32>(0.66,0.78,0.84),vec3<f32>(0.18,0.39,0.65),clamp(ray.z,0.0,1.0))
        + vec3<f32>(1.0,0.84,0.6)*pow(max(dot(ray,sun),0.0),512.0);
}
@fragment fn ocean_fragment(v: Vertex) -> @location(0) vec4<f32> {
    // S256 : empreinte du pixel sur l'eau, puis pentes de la queue spectrale (normales seulement).
    let footprint = max(length(dpdx(v.local.xy)), length(dpdy(v.local.xy)));
    var slope = vec2<f32>(0.0);
    if (p.spectral.w > 0.5) {
        // S260 : pente eulérienne J⁻ᵀ·(∇η_B + ∇η_T) ; repli (det < 0,1) : pente de Lagrange.
        let t = tail_cwm(v.lag, footprint);
        let e = euler_slope(v.slope + t.s, v.g + t.g);
        slope = select(v.slope + t.s, e.xy, e.z >= 0.1);
    } else {
        slope = v.slope + tail_slope(v.local.xy, footprint);
    }
    let n = normalize(vec3<f32>(-slope,1.0));
    let ray = normalize(v.local);
    let fresnel = 0.02+0.98*pow(1.0-max(dot(-ray,n),0.0),5.0);
    let sun = normalize(vec3<f32>(-0.4,0.3,0.8));
    let reflection = reflect(ray,n);
    let glint = pow(max(dot(reflection,sun),0.0),180.0);
    if (p.eye.w > 0.5) {
        // Habillage « ciel clair » : eau bleu profond (photo B), air clair, reflet du soleil plus franc.
        let body = vec3<f32>(0.004,0.060,0.170)*(0.6+0.4*max(dot(n,sun),0.0));
        let clear = mix(body,sky(reflection),fresnel)+vec3<f32>(1.0,0.95,0.85)*glint*1.2;
        return vec4<f32>(mix(clear,CLEAR_HORIZON,1-exp(-length(v.local)/6000.0)),1.0);
    }
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
