# ADR-141 — Surface linéarisée et coefficients temporels de δ

- Statut : acté, S233, 2026-09-14, autonomie technique de REPRISE §2.
- Précise I-08 et ADR-003 pour l'intégration locale ; ne réduit pas ADR-127.

Le candidat à pression possède une hauteur imposée mais ne la fait pas évoluer. Un premier
mode **linéarisé** utilise la même géométrie fixe, sans advection quadratique : projection
avec p=ρg(η−z₀), puis η nouvelle issue de la divergence des débits horizontaux intégrés.
La nouvelle hauteur alimente la pression du prochain pas. C'est une étape vers le δ général,
pas une réception de frontières géométriquement mobiles, cavités, retournements ou 3D.

La durée de ce chemin entre et sort en **microsecondes entières**. Aucun temps absolu ni
accumulation de deltaTime flottant. La durée peut être représentée en f64 pour construire
les coefficients dimensionnés ρ/dt, dt/ρ et dt/dx ; ces coefficients sont arrondis en f32
avant les opérations sur les champs. **Cette conversion de coefficients est autorisée** :
elle précise la frontière temporelle d'I-08 comme ADR-003 l'a fait pour les phases.
Elle n'autorise ni durée/horloge en f32, ni champ ou réduction physique en f64.
Les anciennes API expérimentales prenant dt f32 restent non conformes et ne sont pas
appelées par ce nouveau chemin. Leur migration reste un travail distinct et explicite.

Le mode évolutif ne publie aucun temps avancé si son pas échoue : u/w/p/η sont restaurés
ensemble, y compris sur expiration ou pression non convergée. Il ne transforme pas une
pression dégradée en déplacement de masse reçu. Contrôle coopératif conservé ; I-05 complet
n'est pas acquis par ce contrat. Aucune sérialisation δ, aucune autorité gameplay.

## Note datée du 2026-09-15 (S237) — un mode géométriquement mobile suit le même contrat

`step_surface_mobile` ajoute la surface **géométriquement mobile** (fonction hauteur, fluide fantôme,
advection quadratique, niveau de référence fourni par `set_free_surface`) sans rien retirer à ce mode.
Il applique ce contrat temporel tel quel : durée et budget en microsecondes entières, coefficients
`ρ/dt`, `dt/ρ`, `dt/dx` **et `dt` pour l'advection** construits en f64 puis arrondis en f32, refus
atomiques de u/w/p/η et du reste d'arrondi. La phrase « pas une réception de frontières
géométriquement mobiles » reste vraie de *ce* mode ; le mode mobile est reçu à part, avec ses limites
(surface graphe, pas de mouillage du fond), dans [SURFACE-MOBILE-S237](../validation/SURFACE-MOBILE-S237.md).
