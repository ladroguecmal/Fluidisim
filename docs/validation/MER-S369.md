# La voie d'A289 : δ relatif à la dynamique de B — S369

2026-09-26. **A289** ([angle mort](../registres/ANGLES-MORTS.md), sévérité 3) bloque l'ordre E du lot 2 : sous une houle B
seule, sans perturbation, δ croît jusqu'à trois fois la houle ([MER-S319](MER-S319.md)), à un taux qui ne dépend ni de la
maille ni du pas (§8 de S319). L'utilisateur a délégué la voie, sous trois critères — *« le plus favorable au réalisme
ainsi que les performances, simple »* ([ADR-197](../adr/ADR-197-reponses-du-2026-09-26.md) D6). Machine de référence,
référence CPU de δ ; aucun téléchargement.

## Reproduire

- Commit de P4 de S369 ou plus récent. `cargo run -p water-core --release --offline --example transfert_oriente -- mer
  <maille> <houle>` (E1, `MER_DUREE`) ou `mer_paquet` (E2) ; options `MER_RELATIF=<masque>` (§1), `MER_GERME=<m>` (§3),
  `MER_TRACE`, `MER_PROFIL`, `MER_DT_US`. Bits d'essai du §3 : 8, 16, 32, 64, ajoutés au masque 7.
- Test `zero_delta_stays_zero_under_b_alone_when_relative_s369` (`tests_delta3d.rs`).
- Houle d'une composante, λ = 4 m (`k·h` = 3,1), vers `+x` ; tranche de deux rangées en `y`, 90 m, éponges de 18 m ;
  pas de 10 ms sauf mention. Taux : pente de `ln δ_max` par moindres carrés.

## En une phrase

La croissance de S319 est **forcée par ce que B linéaire laisse de ses propres équations** : retirés du pas couplé, ces
trois termes laissent δ nul au bit sous la houle, gratuitement ; mais derrière cette source se tenait une **instabilité
convective des perturbations** sous une houle raide, qui la remplace comme blocage de l'ordre E.

## 1. D'où vient la croissance : trois termes où δ ne figure pas

Le pas couplé (`delta3d_coupling.rs`, S297) résout la surface **totale** et donne à δ, comme sources, trois termes qui ne
dépendent que de B — le reste de B dans les équations complètes, que SPEC-004 §6.1 demandait de soustraire :

| bit | terme | où |
|---:|---|---|
| 1 | résidu de quantité de mouvement de B, `U_t + (U·∇)U + ∇p/ρ` | `extra3`, prédiction |
| 2 | transport de B entre le plan moyen et **sa propre** surface | la bande, `transport_coupled3` |
| 4 | erreur de pression de B à sa propre surface, `ρ·g·η_B − p_B(repos + η_B)` | les fantômes de surface |

`Volume3::set_relative_background(masque)` les retire : restent les termes **croisés** (B advecte δ, δ advecte B, la bande
et la pression de B entre sa surface et la surface totale) et ceux de δ seul. La surface « propre » de B est formée comme
la totale, pour qu'à δ nul les opérandes soient les mêmes au bit. Critère écrit avant : E1 à 40 s, δ nul au bit.

| E1, 25 cm, houle 5 cm, 40 s | aucun (S322) | 1 | 2 | 3 | 4 | 5 | 6 | **1+2+4** |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| δ maximal | 15,8 cm | 3,06 | 14,0 | 1,45 | 17,5 | 4,80 | 14,7 | **0, au bit** |
| taux, 10 → 30 s (s⁻¹) | 0,101 | 0,074 | 0,099 | 0,047 | 0,095 | 0,066 | 0,108 | — |

**Tenu.** Le résidu de quantité de mouvement porte l'essentiel ; aucun terme seul ne suffit — chacun nourrit la dérive
que les autres laissent. Sans eux, le calcul d'E1 prend 51 s au lieu de ≈ 170 : la pression n'a plus rien à résoudre.
Le test garde la propriété : 150 pas, hauteur, vitesses et pression nulles au bit ; le pas de S297, 3·10⁻³ m.

## 2. Pourquoi cette voie, et non les trois d'A289

- **Réalisme.** Dans le domaine, la mer de fond est B, exactement comme partout ailleurs : aucune couture au bord, la
  cohérence de phase de δ avec B est exacte par construction (4.21). Les perturbations gardent toute leur physique —
  entre elles et avec B. Un **rappel lent** aurait dû amortir toute perturbation plus vite que 0,1 s⁻¹ ; une **durée de
  vie bornée**, recréer les domaines avant dix secondes ; la **dispersion d'amplitude dans B** ne corrigeait que la
  vitesse de phase, pas les harmoniques liés, et le pas couplé ne porte que 0,85 fois celle de Stokes (S274).
- **Performances.** Gratuite : trois termes de moins, une pression qui converge sans itérer sous B seul.
- **Simplicité.** Trois retraits locaux, un drapeau ; B inchangé, donc la mer de tous inchangée.

Ce qu'elle abandonne : les corrections non linéaires de B à lui-même dans le domaine (crêtes de Stokes) — que B n'a nulle
part ailleurs. Rendre B plus réaliste reste possible, et resterait cohérent avec δ.

## 3. Ce qui se tenait derrière : une instabilité des perturbations

Critères écrits avant : un germe de 1 mm ne croît pas (taux < 0,01 s⁻¹ sur 40 s) ; E2, le paquet de l'ordre C (2 cm), reçu
à droite à 30 % du paquet seul (S317).

| masque 1+2+4, 25 cm sauf mention | houle | durée | δ max | croissance |
|---|---:|---:|---:|---|
| germe 1 mm | 5 cm | 40 s | 1,12 mm | aucune — **critère 2 tenu tel qu'écrit** |
| germe 1 mm, prolongé | 5 cm | 95 s | 12,7 mm | dès ≈ 50 s, **0,060 s⁻¹** |
| E2, paquet 2 cm | 5 cm | 95 s | 14,8 cm | dès ≈ 35 s, **0,052 s⁻¹** ; reçu 1 620 fois le paquet |
| E2, pas de 5 ms | 5 cm | 95 s | 14 cm | 0,051 s⁻¹ — le pas n'y est pour rien |
| E2, maille 12,5 cm | 5 cm | 78 s | 10,3 cm | 0,033 s⁻¹ (40 → 78 s) — plus lente à maille fine |
| E2 | 2,5 cm | 95 s | 2,17 cm (départ) | **aucune** ; reçu 21 fois le paquet (§4) |

**Critère 3 manqué.** Le germe croît au même taux que le paquet : **une instabilité linéaire** de δ autour de B, que la
source de S319 masquait. Elle est **convective** : l'écart maximal croît le long du sens de la houle — ×e tous les ≈ 18 m
— jusqu'à l'éponge de sortie. Indépendante du pas de temps ; un peu plus lente à maille fine ; absente sous une houle
deux fois plus douce (`ak` = 0,039) sur 95 s. Son taux, à `ak` = 0,079, vaut quatre fois celui de Benjamin-Feir
(`ω(ak)²/2` = 0,012 s⁻¹) : pas la physique d'une vraie houle.

### Le terme qui la porte

Même bisection qu'au §1, sur les termes **croisés** (bits d'essai 8 à 64, `TRIAL_*` : chacun ampute le couplage, aucun
n'est une physique). Germe de 1 mm, houle de **7,5 cm** (`ak` = 0,118) pour qu'elle se montre avant 60 s ; taux de 35 à
59 s :

| retiré, en plus des trois termes propres | δ à 59 s | taux (s⁻¹) |
|---|---:|---:|
| rien (masque 7) | 32 mm | 0,115 |
| B advecte δ, `U·∇u'` (15) | 206 mm, dès 15 s | — (le couplage n'a plus de sens) |
| **δ advecte B, `u'·∇U` (23)** | **0,9 mm** | **−0,002 : plus d'instabilité** |
| la bande croisée (39) | 10 mm | 0,090 |
| la pression croisée (71) | 56 mm | 0,125 |

**Le terme de cisaillement `u'·∇U` porte l'instabilité** ; les autres la modulent. Son taux croît comme ≈ `a²` (0,115 à
`ak` = 0,118, 0,056 à 0,079, rien de visible à 0,039), vaut ≈ 4,5 fois Benjamin-Feir aux deux amplitudes, et **décroît
quand la maille s'affine** (0,052 → 0,033 de 25 à 12,5 cm) : la signature d'un défaut de discrétisation, pas d'une
physique. Hypothèse, non éprouvée : pour un fond et une perturbation irrotationnels, `U·∇u' + u'·∇U = ∇(U·u')` est un
gradient, que la projection absorbe et qui n'agit qu'à la surface ; discrétisés séparément, explicitement et centrés, les
deux termes ne forment plus un gradient discret et produisent de l'énergie. Remède à éprouver : les termes croisés sous
cette forme de Bernoulli — exacte, gratuite.

## 4. Le critère de volume est mal posé dans une mer

Sous 2,5 cm de houle, sans croissance, la ligne reçoit **21 fois** le volume net du paquet, et la gauche en perd autant :
un transport à travers le domaine. Ordre de grandeur : le transport croisé `a_B·a_δ·ω/2` d'un paquet de 2 cm dans 2,5 cm
de houle, pendant les ≈ 15 s de son passage, vaut ≈ 7·10⁻³ m³ brut ; sur quelques battements (3,9 s), il ne s'annule qu'en
partie — 2,4·10⁻³ m³ restent. Le volume net d'un paquet presque de moyenne nulle (1,2·10⁻⁴ m³) est une quantité trop
petite pour se lire sous ce transport : le critère de S319 §3 doit être refondu (sur la durée, ou sur le transport propre
de δ), pas seulement tenu.

## 5. Limites

Une composante, une tranche de deux rangées, quatre amplitudes ; la croissance après 95 s n'est pas suivie ; la forme de
Bernoulli n'est pas essayée. Le fond plat
et une surface qui ne franchit pas de centre de maille rendent le point fixe exact ; ailleurs, l'erreur de surface est
interpolée entre deux colonnes (ordre `dx²`). La production GPU (`delta3d_step.wgsl`) garde le pas de S297.
