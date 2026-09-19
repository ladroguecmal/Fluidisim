// ADR-172 : opérateur mobile figé, aucun état du rendu. Faces gauche/droite/bas/haut.
struct Row { weights: vec4<f32>, ghosts: vec4<f32> }
struct Params { nx: u32, cells: u32, inv: f32, padding: u32 }
@group(0) @binding(0) var<storage, read> rows: array<Row>;
@group(0) @binding(1) var<storage, read> source: array<f32>;
@group(0) @binding(2) var<storage, read> rhs: array<f32>;
@group(0) @binding(3) var<storage, read_write> dest: array<f32>;
@group(0) @binding(4) var<uniform> params: Params;

fn apply_row(c: u32) -> vec2<f32> {
    var acc = 0.0;
    var diag = 0.0;
    for (var f=0u; f<4u; f++) {
        let a=rows[c].weights[f];
        let g=rows[c].ghosts[f];
        if a==0.0 { continue; }
        if g>0.0 {
            acc += a*source[c]*g;
            diag += a*g;
        } else {
            var j=c;
            switch f {
                case 0u: { j=c-1u; }
                case 1u: { j=c+1u; }
                case 2u: { j=c-params.nx; }
                default: { j=c+params.nx; }
            }
            acc += a*(source[c]-source[j]);
            diag += a;
        }
    }
    return vec2<f32>(acc*params.inv,diag*params.inv);
}
@compute @workgroup_size(64)
fn apply(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x>=params.cells { return; }
    dest[id.x]=apply_row(id.x).x;
}
@compute @workgroup_size(64)
fn relax_pressure(@builtin(global_invocation_id) id: vec3<u32>) {
    let c=id.x;
    if c>=params.cells { return; }
    let op=apply_row(c);
    var value=0.0;
    if op.y>0.0 { value=source[c]+(4.0/5.0)*(1.0/op.y)*(rhs[c]-op.x); }
    dest[c]=value;
}
