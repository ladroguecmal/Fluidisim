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
    rest: f32,
    nz: u32,
    g_eff: f32,
    scale: f32,
    theta_min: f32,
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
// Entrée par colonne puis par maille : perturbation de surface, puis divergence.
@group(0) @binding(5) var<storage, read> cells_in: array<f32>;
// Sortie : surface totale (colonnes), fantôme du haut (colonnes), second membre et
// préconditionneur (mailles), à la suite.
@group(0) @binding(6) var<storage, read_write> cells_out: array<f32>;

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

// ── Le couplage : surface totale, fantômes de fond, second membre ────────────────────────────
//
// Porté de `delta3d_coupling.rs` l. 60-125 et de `ghost_up3` / `ghost_side3`. Le cœur vérifie
// que l'élévation du fond est constante sur la verticale d'une colonne ; ici c'est automatique,
// la carte évaluant `eta` sans dépendance en z — la vérification n'a donc pas d'objet, et son
// absence n'est pas un relâchement.
//
// `eta_roundoff` vaut zéro tant que la surface n'a pas avancé. Ce lot ne porte pas l'avance de
// surface, donc il ne porte pas non plus sa compensation : à reprendre avec la surface mobile.

fn columns() -> u32 { return params.nx * params.ny; }
fn cells() -> u32 { return params.nx * params.ny * params.nz; }

fn u_faces() -> u32 { return (params.nx + 1u) * params.ny * params.nz; }
fn v_faces() -> u32 { return params.nx * (params.ny + 1u) * params.nz; }

/// Champ `f` de la face `w` d'indice (i, j, k), dans le tampon des faces.
fn face_w(i: u32, j: u32, k: u32, f: u32) -> f32 {
    let index = u_faces() + v_faces() + (k * params.ny + j) * params.nx + i;
    return out[index * FIELDS + f];
}
fn face_u(i: u32, j: u32, k: u32, f: u32) -> f32 {
    let index = (k * params.ny + j) * (params.nx + 1u) + i;
    return out[index * FIELDS + f];
}
fn face_v(i: u32, j: u32, k: u32, f: u32) -> f32 {
    let index = u_faces() + (k * (params.ny + 1u) + j) * params.nx + i;
    return out[index * FIELDS + f];
}

const F_ETA: u32 = 0u;
const F_P_DYN: u32 = 19u;
const F_GRAD_P_X: u32 = 20u;
const F_GRAD_P_Y: u32 = 21u;
const F_GRAD_P_Z: u32 = 22u;

fn total_height(c: u32) -> f32 { return cells_out[c]; }

/// Surface totale et fantôme du haut, une invocation par colonne.
@compute @workgroup_size(64)
fn couple_columns(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= columns()) { return; }
    let i = c % params.nx;
    let j = c / params.nx;
    let elevation = face_w(i, j, 0u, F_ETA);
    let height = cells_in[c] + elevation;
    cells_out[c] = height;

    var ghost = 0.0;
    // Dernière maille mouillée, balayée du haut comme le cœur ; nz est petit.
    var k = params.nz;
    loop {
        if (k == 0u) { break; }
        k = k - 1u;
        if ((f32(k) + 0.5) * params.dx < height) {
            let dz = height - f32(k + 1u) * params.dx;
            let eta_bg = face_w(i, j, k + 1u, F_ETA);
            let p = face_w(i, j, k + 1u, F_P_DYN);
            let gz = face_w(i, j, k + 1u, F_GRAD_P_Z);
            ghost = params.rho * params.g_eff * eta_bg - (p + dz * gz);
            break;
        }
    }
    cells_out[columns() + c] = ghost;
}

/// Second membre couplé et préconditionneur, une invocation par maille. Même parcours de faces
/// et même ordre qu'en 3D dans le cœur ; les valeurs de fantôme portent en plus le fond.
@compute @workgroup_size(64)
fn couple_rhs(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = id.x;
    if (c >= cells()) { return; }
    let plane = columns();
    let i = c % params.nx;
    let j = (c / params.nx) % params.ny;
    let k = c / plane;
    let col = j * params.nx + i;

    let base_rhs = 2u * plane;
    let base_prec = 2u * plane + cells();
    let h = total_height(col);
    let zc = (f32(k) + 0.5) * params.dx;
    if (zc >= h) {
        cells_out[base_rhs + c] = 0.0;
        cells_out[base_prec + c] = 0.0;
        return;
    }

    var b = params.scale * cells_in[plane + c];
    var diag = 0.0;
    let inv = 1.0 / (params.dx * params.dx);
    // Fantôme latéral : pression hydrostatique locale plus le fond, pris sur la face.
    let side_base = params.rho * params.g_eff * (zc - params.rest);

    // x−, x+, y−, y+ dans l'ordre du cœur.
    for (var face = 0u; face < 4u; face = face + 1u) {
        var ok = false;
        var other_col = col;
        var bg_p = 0.0;
        var bg_g = 0.0;
        var sign = 1.0;
        if (face == 0u && i > 0u) {
            ok = true; other_col = col - 1u;
            // Face d'indice max(i, i−1) = i ; notre maille est la plus haute, donc sign = −1.
            bg_p = face_u(i, j, k, F_P_DYN); bg_g = face_u(i, j, k, F_GRAD_P_X); sign = -1.0;
        } else if (face == 1u && i + 1u < params.nx) {
            ok = true; other_col = col + 1u;
            bg_p = face_u(i + 1u, j, k, F_P_DYN); bg_g = face_u(i + 1u, j, k, F_GRAD_P_X); sign = 1.0;
        } else if (face == 2u && j > 0u) {
            ok = true; other_col = col - params.nx;
            bg_p = face_v(i, j, k, F_P_DYN); bg_g = face_v(i, j, k, F_GRAD_P_Y); sign = -1.0;
        } else if (face == 3u && j + 1u < params.ny) {
            ok = true; other_col = col + params.nx;
            bg_p = face_v(i, j + 1u, k, F_P_DYN); bg_g = face_v(i, j + 1u, k, F_GRAD_P_Y); sign = 1.0;
        }
        if (!ok) { continue; }
        let other = total_height(other_col);
        if (zc < other) {
            diag = diag + 1.0;
        } else {
            let theta = max((h - zc) / (h - other), params.theta_min);
            let a = 1.0 / theta;
            diag = diag + a;
            let bg = -(bg_p + sign * (theta - 0.5) * params.dx * bg_g);
            b = b + (side_base + bg) * a * inv;
        }
    }
    if (k > 0u) { diag = diag + 1.0; }
    if (k + 1u < params.nz && (f32(k + 1u) + 0.5) * params.dx < h) {
        diag = diag + 1.0;
    } else {
        let a = 1.0 / max((h - zc) / params.dx, params.theta_min);
        diag = diag + a;
        // `eta[c] − repos` porte la perturbation seule ; le fond arrive par `ghost_up`.
        let value = params.rho * params.g_eff * (cells_in[col] - params.rest)
            + cells_out[plane + col];
        b = b + value * a * inv;
    }

    cells_out[base_rhs + c] = b;
    if (diag > 0.0) { cells_out[base_prec + c] = 1.0 / (diag * inv); } else { cells_out[base_prec + c] = 0.0; }
}
