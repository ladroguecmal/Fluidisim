// S300 / ADR-175 D1 — le **fond B évalué sur la carte**.
//
// Le CPU ne publie que les paramètres analytiques : par composante, amplitude, nombre d'onde,
// direction, et la phase temporelle déjà repliée. Cette dernière ne dépend pas du point, et son
// calcul passe par 128 bits dans le cœur, ce que WGSL n'a pas ; elle est donc calculée une fois
// par composante et par pas, côté hôte. Tout le reste — phase spatiale, sinus et cosinus,
// atténuation — vit ici, et ne coûte au CPU aucun travail proportionnel aux mailles.
//
// Les trois primitives sont portées **à l'identique** du cœur : même réduction d'angle Q32,
// mêmes polynômes, même exponentielle par mise à l'échelle sur les bits IEEE. Aucune identité
// au bit n'est promise pour autant (ADR-175 D4) : ce qui est promis, c'est que l'écart mesuré
// vienne de l'arithmétique de la carte, pas d'une formule différente.

struct Params {
    count: u32,
    probes: u32,
    _pad0: u32,
    _pad1: u32,
    rho: f32,
    gravity: f32,
    _pad2: f32,
    _pad3: f32,
};

// (amplitude, k en tours/m, dir.x, dir.y)
@group(0) @binding(0) var<storage, read> components: array<vec4<f32>>;
// Phase temporelle repliée, une par composante, réécrite à chaque instant.
@group(0) @binding(1) var<storage, read> time_phase: array<u32>;
// Points d'interrogation, locaux à l'ancre de B : (x, y, z, inutilisé).
@group(0) @binding(2) var<storage, read> points: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read_write> out: array<f32>;
@group(0) @binding(4) var<uniform> params: Params;

const TWO_POW_32: f32 = 4294967296.0;
const FRAC_PI_2: f32 = 1.5707963267948966;
const FRAC_PI_4: f32 = 0.7853981633974483;

/// `PhaseQ32::from_distance` : fraction de tour de `k·d`, en virgule fixe 2³².
///
/// **La fraction se prend en entier, pas en flottant.** Mesuré S300 sur cette carte : le
/// `x − floor(x)` compilé rend un résultat distant d'un ulp de celui du CPU, alors même que le
/// produit `k·d` est identique au bit. Près de la borne d'I-08 (`|d| < 4096 m`), `k·d` vaut
/// plusieurs milliers de tours et il ne reste qu'une poignée de bits à la fraction : un ulp y
/// pèse **10⁻³ de tour**, soit des millièmes d'amplitude sur le sinus. Le découpage binaire
/// ci-dessous est exact et ne dépend d'aucune optimisation du compilateur.
///
/// `x = mantisse · 2^(e−23)`, donc `x · 2³² = mantisse · 2^(e+9)`, pris modulo 2³². Le signe
/// se traite par complément : la phase est une fraction de tour, donc cyclique.
fn phase_from_distance(k_turns_per_m: f32, d: f32) -> u32 {
    let turns = k_turns_per_m * d;
    let bits = bitcast<u32>(turns);
    let exponent = i32((bits >> 23u) & 0xffu);
    // Zéro et sous-normaux : la fraction pèse moins d'une unité Q32.
    if (exponent == 0) { return 0u; }
    let mantissa = (bits & 0x7fffffu) | 0x800000u;
    let shift = exponent - 127 + 9;
    var magnitude = 0u;
    if (shift >= 0) {
        if (shift < 32) { magnitude = mantissa << u32(shift); }
    } else {
        if (shift > -24) { magnitude = mantissa >> u32(-shift); }
    }
    if ((bits >> 31u) == 1u) { return 0u - magnitude; }
    return magnitude;
}

/// `PhaseQ32::sin_cos` : quadrant, repliement sur π/4, deux Horner.
fn phase_sin_cos(phase: u32) -> vec2<f32> {
    let quadrant = phase >> 30u;
    let within = phase & 0x3fffffffu;
    let x = f32(within) * (FRAC_PI_2 / 1073741824.0);
    let swap = x > FRAC_PI_4;
    var a = x;
    if (swap) { a = FRAC_PI_2 - x; }
    let a2 = a * a;

    // Même suite d'opérations que `poly_sin` du cœur, dans le même ordre.
    var ps = 1.0 / 362880.0;
    ps = ps * a2 - 1.0 / 5040.0;
    ps = ps * a2 + 1.0 / 120.0;
    ps = ps * a2 - 1.0 / 6.0;
    ps = ps * a2;
    let s0 = a * ps + a;

    // Idem `poly_cos`.
    var pc = -1.0 / 3628800.0;
    pc = pc * a2 + 1.0 / 40320.0;
    pc = pc * a2 - 1.0 / 720.0;
    pc = pc * a2 + 1.0 / 24.0;
    pc = pc * a2 - 1.0 / 2.0;
    let c0 = pc * a2 + 1.0;

    var s = s0;
    var c = c0;
    if (swap) { s = c0; c = s0; }
    switch quadrant {
        case 0u: { return vec2<f32>(s, c); }
        case 1u: { return vec2<f32>(c, -s); }
        case 2u: { return vec2<f32>(-s, -c); }
        default: { return vec2<f32>(-c, s); }
    }
}

/// `attenuation` : `exp(−x)` pour `x ≥ 0`, Taylor de degré 10 sur [0, ln2] et mise à l'échelle
/// `2⁻ⁿ` reconstruite depuis les bits IEEE. Même découpage de `ln 2` que le cœur.
fn attenuation(x: f32) -> f32 {
    if (x >= 104.0) { return 0.0; }
    // `core::f32::consts::LN_2`, tel que le cœur le divise.
    let n = u32(x / 0.6931472);
    if (n > 149u) { return 0.0; }
    let r = (x - f32(n) * 0.69314575) - f32(n) * 0.0000014286068;
    var p = 1.0;
    for (var j = 10; j >= 1; j = j - 1) {
        p = 1.0 - r * p / f32(j);
    }
    var scale = 0.0;
    if (n <= 126u) {
        scale = bitcast<f32>((127u - n) << 23u);
    } else {
        scale = bitcast<f32>(1u << (149u - n));
    }
    return p * scale;
}

/// Banc P2 : les trois primitives seules. `points[i]` porte `(k, d, x, _)` ; la sortie donne
/// la phase (bits réinterprétés), son sinus, son cosinus, et l'atténuation de `x`.
@compute @workgroup_size(64)
fn primitives(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    if (i >= params.probes) { return; }
    let q = points[i];
    let phase = phase_from_distance(q.x, q.y);
    let sc = phase_sin_cos(phase);
    out[i * 6u + 0u] = bitcast<f32>(phase);
    out[i * 6u + 1u] = sc.x;
    out[i * 6u + 2u] = sc.y;
    out[i * 6u + 3u] = attenuation(q.z);
    // Diagnostic : le produit brut et sa partie fractionnaire, pour savoir si un desaccord
    // vient de la multiplication elle-meme ou de ce qui suit.
    let turns = q.x * q.y;
    out[i * 6u + 4u] = turns;
    out[i * 6u + 5u] = turns - floor(turns);
}
