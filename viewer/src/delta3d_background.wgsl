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
    nx: u32,
    ny: u32,
    rho: f32,
    gravity: f32,
    dx: f32,
    _pad0: f32,
    nz: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
    // Coin bas du domaine δ, local à l'ancre de B.
    origin: vec4<f32>,
};

// Deux vec4 par composante : (amplitude, k en tours/m, dir.x, dir.y) puis (omega, k en rad/m,
// 0, 0). `omega` se calcule en f64 dans le cœur — absent de WGSL — donc l'hôte le publie ;
// c'est un paramètre de composante, pas un échantillon.
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

// ── Le champ complet ─────────────────────────────────────────────────────────────────────────
//
// 26 flottants par point, dans l'ordre de `BackgroundSample` : eta, grad_eta (3), u (3),
// du_dt (3), grad_u (9, ligne par ligne), p_dyn, grad_p_dyn (3), laplacian_u (3).
//
// Deux corps, comme le cœur : sous le plan moyen l'atténuation `e = exp(kz)` (ADR-113), au-dessus
// le prolongement linéaire `m = 1 + kz` d'ADR-154 §2. La bascule est `local.z > 0`, et hors du
// domaine d'I-08 le point est refusé — ici en rendant un marqueur, l'hôte ne pouvant pas
// recevoir d'erreur d'un noyau.

const FIELDS: u32 = 26u;

fn admits_local(p: vec3<f32>) -> bool {
    return abs(p.x) < 4096.0 && abs(p.y) < 4096.0 && abs(p.z) < 4096.0;
}

@compute @workgroup_size(64)
fn sample_field(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    if (i >= params.probes) { return; }
    let base = i * FIELDS;
    let local = points[i].xyz;

    var eta = 0.0;
    var p_dyn = 0.0;
    var grad_eta = vec3<f32>(0.0);
    var u = vec3<f32>(0.0);
    var du_dt = vec3<f32>(0.0);
    var grad_p_dyn = vec3<f32>(0.0);
    var laplacian_u = vec3<f32>(0.0);
    var grad_u0 = vec3<f32>(0.0);
    var grad_u1 = vec3<f32>(0.0);
    var grad_u2 = vec3<f32>(0.0);

    let admis = admits_local(local);
    let above = admis && local.z > 0.0;
    // Le cœur refuse un point hors domaine ou au-dessus du plan sans prolongement ; le marqueur
    // laisse l'hôte le constater sans inventer de valeur.
    if (!admis) {
        for (var f = 0u; f < FIELDS; f = f + 1u) { out[base + f] = bitcast<f32>(0x7fc00000u); }
        return;
    }

    for (var n = 0u; n < params.count; n = n + 1u) {
        let c0 = components[n * 2u];
        let c1 = components[n * 2u + 1u];
        let amplitude = c0.x;
        let dir = vec2<f32>(c0.z, c0.w);
        let omega = c1.x;
        let k = c1.y;

        let d = local.x * dir.x + local.y * dir.y;
        let phase = phase_from_distance(c0.y, d) + time_phase[n];
        let sc = phase_sin_cos(phase);
        let sn = sc.x;
        let cs = sc.y;

        var e = 1.0;
        var m = 1.0;
        if (above) { m = 1.0 + k * local.z; } else { e = attenuation(-k * local.z); }
        let a = select(amplitude * omega * e, amplitude * omega, above);
        let pressure_gradient = select(params.rho * params.gravity * amplitude * e * k,
                                       params.rho * params.gravity * amplitude * k, above);
        // Sous le plan : Δ = k²(1 − |d|²). Au-dessus, aucune courbure en z : Δ = −k²|d|².
        let lap = select((k * k) * ((1.0 - dir.x * dir.x) - dir.y * dir.y),
                         -(k * k) * (dir.x * dir.x + dir.y * dir.y), above);

        eta = eta + amplitude * sn;
        let slope = amplitude * k * cs;
        // Boucles déroulées : FXC refuse l'indexation dynamique d'un vecteur en écriture.
        // L'ordre entre cibles distinctes est sans effet — chacune n'accumule qu'une fois.
        grad_eta.x = grad_eta.x + slope * dir.x;
        grad_eta.y = grad_eta.y + slope * dir.y;
        u.x = u.x + a * sn * dir.x;
        u.y = u.y + a * sn * dir.y;
        du_dt.x = du_dt.x - a * omega * cs * dir.x;
        du_dt.y = du_dt.y - a * omega * cs * dir.y;
        laplacian_u.x = laplacian_u.x + lap * (a * sn * dir.x);
        laplacian_u.y = laplacian_u.y + lap * (a * sn * dir.y);
        grad_u0.x = grad_u0.x + a * k * cs * dir.x * dir.x;
        grad_u0.y = grad_u0.y + a * k * cs * dir.x * dir.y;
        grad_u1.x = grad_u1.x + a * k * cs * dir.y * dir.x;
        grad_u1.y = grad_u1.y + a * k * cs * dir.y * dir.y;
        if (above) {
            grad_p_dyn.x = grad_p_dyn.x + pressure_gradient * m * cs * dir.x;
            grad_p_dyn.y = grad_p_dyn.y + pressure_gradient * m * cs * dir.y;
            grad_u2.x = grad_u2.x + a * k * m * sn * dir.x;
            grad_u2.y = grad_u2.y + a * k * m * sn * dir.y;
            u.z = u.z - a * m * cs;
            du_dt.z = du_dt.z - a * omega * m * sn;
            grad_u2.z = grad_u2.z - a * k * cs;
            p_dyn = p_dyn + params.rho * params.gravity * amplitude * m * sn;
            grad_p_dyn.z = grad_p_dyn.z + pressure_gradient * sn;
            laplacian_u.z = laplacian_u.z + lap * (-a * m * cs);
        } else {
            grad_p_dyn.x = grad_p_dyn.x + pressure_gradient * cs * dir.x;
            grad_p_dyn.y = grad_p_dyn.y + pressure_gradient * cs * dir.y;
            // Sous le plan, la ligne i reçoit aussi sa composante z, et la ligne z la sienne.
            grad_u0.z = grad_u0.z + a * k * sn * dir.x;
            grad_u2.x = grad_u2.x + a * k * sn * dir.x;
            grad_u1.z = grad_u1.z + a * k * sn * dir.y;
            grad_u2.y = grad_u2.y + a * k * sn * dir.y;
            u.z = u.z - a * cs;
            du_dt.z = du_dt.z - a * omega * sn;
            grad_u2.z = grad_u2.z - a * k * cs;
            p_dyn = p_dyn + params.rho * params.gravity * amplitude * e * sn;
            grad_p_dyn.z = grad_p_dyn.z + pressure_gradient * sn;
            laplacian_u.z = laplacian_u.z + lap * (-a * cs);
        }
    }

    out[base + 0u] = eta;
    out[base + 1u] = grad_eta.x;
    out[base + 2u] = grad_eta.y;
    out[base + 3u] = grad_eta.z;
    out[base + 4u] = u.x;
    out[base + 5u] = u.y;
    out[base + 6u] = u.z;
    out[base + 7u] = du_dt.x;
    out[base + 8u] = du_dt.y;
    out[base + 9u] = du_dt.z;
    out[base + 10u] = grad_u0.x;
    out[base + 11u] = grad_u0.y;
    out[base + 12u] = grad_u0.z;
    out[base + 13u] = grad_u1.x;
    out[base + 14u] = grad_u1.y;
    out[base + 15u] = grad_u1.z;
    out[base + 16u] = grad_u2.x;
    out[base + 17u] = grad_u2.y;
    out[base + 18u] = grad_u2.z;
    out[base + 19u] = p_dyn;
    out[base + 20u] = grad_p_dyn.x;
    out[base + 21u] = grad_p_dyn.y;
    out[base + 22u] = grad_p_dyn.z;
    out[base + 23u] = laplacian_u.x;
    out[base + 24u] = laplacian_u.y;
    out[base + 25u] = laplacian_u.z;
}

// ── Les trois familles de faces MAC ──────────────────────────────────────────────────────────
//
// Même géométrie que `BackgroundGrid3` du cœur (S298) : la face d'axe `a` est décalée d'un
// demi-pas sur les deux autres axes, et pleine sur le sien. Les trois familles sont écrites à
// la suite dans `out`, dans l'ordre u, v, w — comme le cœur les range.
//
// C'est ici que le fond cesse d'être un banc : ces échantillons sont ceux que le second membre
// couplé consomme, et le CPU n'en calcule aucun.

fn faces_of(axis: u32) -> vec3<u32> {
    return vec3<u32>(
        params.nx + u32(axis == 0u),
        params.ny + u32(axis == 1u),
        params.nz + u32(axis == 2u),
    );
}

fn write_sample(base: u32, local: vec3<f32>) {
    var eta = 0.0;
    var p_dyn = 0.0;
    var grad_eta = vec3<f32>(0.0);
    var u = vec3<f32>(0.0);
    var du_dt = vec3<f32>(0.0);
    var grad_p_dyn = vec3<f32>(0.0);
    var laplacian_u = vec3<f32>(0.0);
    var grad_u0 = vec3<f32>(0.0);
    var grad_u1 = vec3<f32>(0.0);
    var grad_u2 = vec3<f32>(0.0);

    if (!admits_local(local)) {
        for (var f = 0u; f < FIELDS; f = f + 1u) { out[base + f] = bitcast<f32>(0x7fc00000u); }
        return;
    }
    let above = local.z > 0.0;

    for (var n = 0u; n < params.count; n = n + 1u) {
        let c0 = components[n * 2u];
        let c1 = components[n * 2u + 1u];
        let amplitude = c0.x;
        let dir = vec2<f32>(c0.z, c0.w);
        let omega = c1.x;
        let k = c1.y;

        let d = local.x * dir.x + local.y * dir.y;
        let phase = phase_from_distance(c0.y, d) + time_phase[n];
        let sc = phase_sin_cos(phase);
        let sn = sc.x;
        let cs = sc.y;

        var e = 1.0;
        var m = 1.0;
        if (above) { m = 1.0 + k * local.z; } else { e = attenuation(-k * local.z); }
        let a = select(amplitude * omega * e, amplitude * omega, above);
        let pressure_gradient = select(params.rho * params.gravity * amplitude * e * k,
                                       params.rho * params.gravity * amplitude * k, above);
        let lap = select((k * k) * ((1.0 - dir.x * dir.x) - dir.y * dir.y),
                         -(k * k) * (dir.x * dir.x + dir.y * dir.y), above);

        eta = eta + amplitude * sn;
        let slope = amplitude * k * cs;
        grad_eta.x = grad_eta.x + slope * dir.x;
        grad_eta.y = grad_eta.y + slope * dir.y;
        u.x = u.x + a * sn * dir.x;
        u.y = u.y + a * sn * dir.y;
        du_dt.x = du_dt.x - a * omega * cs * dir.x;
        du_dt.y = du_dt.y - a * omega * cs * dir.y;
        laplacian_u.x = laplacian_u.x + lap * (a * sn * dir.x);
        laplacian_u.y = laplacian_u.y + lap * (a * sn * dir.y);
        grad_u0.x = grad_u0.x + a * k * cs * dir.x * dir.x;
        grad_u0.y = grad_u0.y + a * k * cs * dir.x * dir.y;
        grad_u1.x = grad_u1.x + a * k * cs * dir.y * dir.x;
        grad_u1.y = grad_u1.y + a * k * cs * dir.y * dir.y;
        if (above) {
            grad_p_dyn.x = grad_p_dyn.x + pressure_gradient * m * cs * dir.x;
            grad_p_dyn.y = grad_p_dyn.y + pressure_gradient * m * cs * dir.y;
            grad_u2.x = grad_u2.x + a * k * m * sn * dir.x;
            grad_u2.y = grad_u2.y + a * k * m * sn * dir.y;
            u.z = u.z - a * m * cs;
            du_dt.z = du_dt.z - a * omega * m * sn;
            grad_u2.z = grad_u2.z - a * k * cs;
            p_dyn = p_dyn + params.rho * params.gravity * amplitude * m * sn;
            grad_p_dyn.z = grad_p_dyn.z + pressure_gradient * sn;
            laplacian_u.z = laplacian_u.z + lap * (-a * m * cs);
        } else {
            grad_p_dyn.x = grad_p_dyn.x + pressure_gradient * cs * dir.x;
            grad_p_dyn.y = grad_p_dyn.y + pressure_gradient * cs * dir.y;
            grad_u0.z = grad_u0.z + a * k * sn * dir.x;
            grad_u2.x = grad_u2.x + a * k * sn * dir.x;
            grad_u1.z = grad_u1.z + a * k * sn * dir.y;
            grad_u2.y = grad_u2.y + a * k * sn * dir.y;
            u.z = u.z - a * cs;
            du_dt.z = du_dt.z - a * omega * sn;
            grad_u2.z = grad_u2.z - a * k * cs;
            p_dyn = p_dyn + params.rho * params.gravity * amplitude * e * sn;
            grad_p_dyn.z = grad_p_dyn.z + pressure_gradient * sn;
            laplacian_u.z = laplacian_u.z + lap * (-a * cs);
        }
    }

    out[base + 0u] = eta;
    out[base + 1u] = grad_eta.x;
    out[base + 2u] = grad_eta.y;
    out[base + 3u] = grad_eta.z;
    out[base + 4u] = u.x;
    out[base + 5u] = u.y;
    out[base + 6u] = u.z;
    out[base + 7u] = du_dt.x;
    out[base + 8u] = du_dt.y;
    out[base + 9u] = du_dt.z;
    out[base + 10u] = grad_u0.x;
    out[base + 11u] = grad_u0.y;
    out[base + 12u] = grad_u0.z;
    out[base + 13u] = grad_u1.x;
    out[base + 14u] = grad_u1.y;
    out[base + 15u] = grad_u1.z;
    out[base + 16u] = grad_u2.x;
    out[base + 17u] = grad_u2.y;
    out[base + 18u] = grad_u2.z;
    out[base + 19u] = p_dyn;
    out[base + 20u] = grad_p_dyn.x;
    out[base + 21u] = grad_p_dyn.y;
    out[base + 22u] = grad_p_dyn.z;
    out[base + 23u] = laplacian_u.x;
    out[base + 24u] = laplacian_u.y;
    out[base + 25u] = laplacian_u.z;
}

@compute @workgroup_size(64)
fn sample_faces(@builtin(global_invocation_id) id: vec3<u32>) {
    let slot = id.x;
    if (slot >= params.probes) { return; }
    // `probes` porte ici le total des trois familles ; on retrouve l'axe par soustraction.
    var axis = 0u;
    var rest = slot;
    loop {
        let dims = faces_of(axis);
        let size = dims.x * dims.y * dims.z;
        if (rest < size || axis == 2u) { break; }
        rest = rest - size;
        axis = axis + 1u;
    }
    let dims = faces_of(axis);
    let i = rest % dims.x;
    let j = (rest / dims.x) % dims.y;
    let k = rest / (dims.x * dims.y);
    // Décalage d'un demi-pas sur les axes autres que celui de la face, comme le cœur.
    let half = vec3<f32>(
        select(0.5, 0.0, axis == 0u),
        select(0.5, 0.0, axis == 1u),
        select(0.5, 0.0, axis == 2u),
    );
    let local = params.origin.xyz + (vec3<f32>(f32(i), f32(j), f32(k)) + half) * params.dx;
    write_sample(slot * FIELDS, local);
}
