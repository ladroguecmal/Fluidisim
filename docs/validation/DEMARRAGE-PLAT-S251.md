# Démarrage couplé plat — S251

## Protocole et diagnostic avant correctif

Reprendre exactement les cas A283 de S250 (16×8 et 32×16), couvercle homogène,
v=0, B/W réels et éponge. Assembler une projection indépendante f64 par arêtes
à partir du même prédicteur f32 ; CG à résidu relatif 10⁻¹³, vrai résidu vérifié
à 10⁻¹¹. Comparer la vitesse, pas seulement la pression. Puis éprouver une projection
du défaut de divergence directement sur la vitesse corrigée, en f32.

| nx | max vitesse prédite | max vitesse oracle | D avant | erreur vitesse relative avant | D après correction | erreur vitesse après |
|---|---|---|---|---|---|---|
| 16 | 1,4033·10⁻⁵ | 2,4695·10⁻⁷ | 1,4173·10⁻⁴ | 1,2819·10⁻⁴ | 2,8772·10⁻⁸ | 9,4477·10⁻⁷ |
| 32 | 1,4035·10⁻⁵ | 3,2145·10⁻⁷ | 8,0636·10⁻⁵ | 5,0539·10⁻⁵ | 4,4209·10⁻⁸ | 2,1468·10⁻⁶ |

Le solveur initial atteint le plancher de pression. La projection retranche environ
98 % de la vitesse prédite : l'erreur absolue de cette soustraction devient visible
rapportée à la petite vitesse restante. Une correction appliquée **à cette vitesse**
abaisse D de trois ordres sans f64 dans le cœur et sans toucher aux tolérances.
46/87 itérations supplémentaires dans ce montage ; pas de boucle de retentatives.
Ce diagnostic ne prouve pas qu'un unique affinage suffira à toute entrée.

Réception avant construction : D≤10⁻⁵ (S199), erreur vitesse contre oracle ≤10⁻⁴
(norme maximale relative, même exigence que le test diagnostic), cas sans correctif
refusé, cas avec correctif reçu ; trajectoire de vingt pas S250, rollback et allocations,
suite complète et empreinte historique. Surface mobile et B4 global hors lot.

Commande diagnostic :
`cargo test --release --offline --manifest-path code/Cargo.toml -p water-core diagnose_flat_projection_s251 -- --nocapture`.
