// S211 : image cosmétique seulement. B et W proviennent du cœur.
struct Params {
    eye: vec4<f32>, forward: vec4<f32>, right: vec4<f32>, up: vec4<f32>,
    impact: vec4<f32>, info: vec4<f32>,
}
@group(0) @binding(0) var<uniform> p: Params;
@group(0) @binding(1) var<storage, read> waves: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read> profile: array<vec2<f32>>;
@group(1) @binding(0) var<storage, read> probes: array<vec4<f32>>;
@group(1) @binding(1) var<storage, read_write> results: array<vec4<f32>>;

// Hauteur et deux pentes. Une seule fonction pour sommets et réception compute.
fn water(q: vec2<f32>) -> vec3<f32> {
    var v = vec3<f32>(0.0);
    for (var i = 0u; i < u32(p.info.x); i++) {
        let c = waves[i];
        let phase = dot(c.yz, q) + c.w;
        v += vec3<f32>(c.x * sin(phase), c.x * cos(phase) * c.yz);
    }
    let d = q - p.impact.xy;
    let r = length(d);
    if (p.info.w > 0.5 && r <= p.impact.z) {
        let x = r / p.impact.w;
        let i = u32(x);
        let t = x - f32(i);
        let t2 = t*t;
        let t3 = t2*t;
        let a = profile[i]; let b = profile[i+1u];
        let m0 = a.y*p.impact.w; let m1 = b.y*p.impact.w;
        let h = (2*t3-3*t2+1)*a.x + (t3-2*t2+t)*m0 + (-2*t3+3*t2)*b.x + (t3-t2)*m1;
        let slope = ((6*t2-6*t)*a.x+(3*t2-4*t+1)*m0+(-6*t2+6*t)*b.x+(3*t2-2*t)*m1)/p.impact.w;
        var gradient = vec2<f32>(0.0);
        if (r > 0.0) { gradient = slope*d/r; }
        v += vec3<f32>(h, gradient);
    }
    return v;
}
@compute @workgroup_size(64)
fn verify(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x < arrayLength(&probes)) { results[id.x] = vec4<f32>(water(probes[id.x].xy), 1.0); }
}
struct Vertex { @builtin(position) clip: vec4<f32>, @location(0) local: vec3<f32>, @location(1) slope: vec2<f32> }
@vertex fn ocean_vertex(@builtin(vertex_index) id: u32) -> Vertex {
    let nx = u32(p.info.y); let ny = u32(p.info.z);
    let x = (f32(id % nx)/f32(nx-1u)*2-1)*1.18;
    let horizon = clamp(-p.forward.z/(p.up.z*p.forward.w), -0.95, 1.2);
    let y = mix(-1.18, horizon-0.003, f32(id/nx)/f32(ny-1u));
    let ray = p.forward.xyz + p.right.xyz*x*p.forward.w*p.right.w + p.up.xyz*y*p.forward.w;
    let distance = min(p.eye.z/max(-ray.z,0.00001),1500.0);
    let q = ray.xy*distance;
    let w = water(q);
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
