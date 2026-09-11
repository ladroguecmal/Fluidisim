# Composition différentielle B+impacts+pressions — S181

S180-1/A50 ; [ADR-117](../adr/ADR-117-composition-differentielle-mixte.md).

## Protocole déclaré avant code

1. Réductions aux fournisseurs reçus, fond compté une fois, pression appliquée une fois.
2. Fond + deux impacts + pression mobile : dérivées spatiales/temporelles et source
   par gradient de Bernoulli ; contre-épreuve de somme des sources isolées.
3. Même champ depuis grande ancre monde ; répétabilité et raccord de surface.
4. Contexte physique, repère et temps communs ; refus tardifs, capacités et pente,
   publication atomique, queue préservée, lot vide contrôlé.
5. Workspace, tests ciblés release, C02/C18 ; ne pas déplacer les références.

## Résultats

Quatre nouveaux tests reçus en debug et release. La requête est
`prepared_water::mixed::differential_world_batch`, sur les publications existantes.
Le résultat DifferentialSample garde la densité du montage ; momentum_residual(nu)
calcule la source continue à soustraire, après composition.

- B seul identique au fournisseur B ; B+un impact identique à S179 ; B+pression et
  pression seule (fond calme) reçus. Somme des champs identique, pression imposée et
  son gradient comptés une fois. À la surface, eta/u gardent les bits de la requête
  mixte historique ; p_dyn=rho*g*eta+p_appliquée reçu à0,001Pa. Répétabilité exacte.
- Fond16 composantes, deux impacts décalés et deux pressions mobiles déjà agrégées
  par le contrôleur : à0,75s et (0,75;0,25;-0,5)m, différences spatiales à1/64 et
  1/128m (exactement représentables par WorldPos), temporelles à±2ms. Budgets :
  grad_u et du_dt 2e-5, grad_p 0,01Pa/m, source via gradient de Bernoulli 3e-5m/s².
  La différence entre source totale et somme des sources isolées égale la somme
  explicite des interactions à1e-7m/s² ; au moins une interaction dépasse1e-6m/s².
  Ce témoin empêche qu'un montage presque inactif certifie l'oubli des termes croisés.
- Ancre (1e9;-1e9;0)m, conversion monde/local reçue. Mauvais repère, gravité ou
  densité, publication décalée d'une microseconde, impacts expirés et journal avec
  perte connue refusés, y compris sur lot vide. Correspondance journal/champs rompue
  détectée ; aucun remplacement silencieux par zéro.
- Point tardif hors emprise, z positif, z=-4096m et coordonnée monde extrême refusés
  sans toucher la sortie. Capacité et max_slope invalide reçus ; pente trop petite
  refusée, lot valide accepté. Scratch peut changer ; queue de sortie préservée.
- La somme linéaire et la décision de pente sont partagées avec les chemins antérieurs.
  Aucune allocation de requête par inspection ; les vecteurs des tests sont des oracles.

Workspace :328 tests réussis, cinq ignorés (235+93 réussis,2+3 ignorés).
C18 hash0x85c8bc610f551d11 et C02 hash0x0a3a3bcc945db263 inchangés, zéro échec.
Le compteur historique alloc_post_seal=1 de ces scénarios n'est pas une mesure de
la nouvelle requête ; aucun budget d'exécution n'est reçu par ce lot.

Commandes depuis code/ :

```text
cargo test -p water-core prepared_water::mixed::tests::differential -- --nocapture
cargo test -p water-core prepared_water::mixed::tests::differential --release
cargo test --workspace
cargo run -p water-harness --release -- check scenarios/C18-invariants.toml scenarios/C02-dispersion.toml
```

## Portée et suite

S180-1 réalisée sur les vues publiées du montage existant, avec entrée monde.
A50/B4 restent partiels : les tests ne reçoivent pas le solveur perturbatif, la surface
libre non linéaire, les coûts ou le déterminisme entre plateformes. L'identité de
repère est contrôlée ; la géométrie de son association à B reste déclarée par l'hôte.

**S181-1, priorité S182 :** recevoir le nouveau consommateur dans le cycle vivant
mixte : réactualisation de pression, renouvellement d'impact, refus/reprise et rejeu,
dérivées et source comparées à une préparation directe au même instant. Puis coût et
consommation perturbative. Porteur : construction BILAN-S145/BILAN-B4-S176.

**Suivi S182 — 2026-09-12 :** S181-1 réalisée sur le montage de bibliothèque,
[CYCLE-DIFFERENTIEL-S182](CYCLE-DIFFERENTIEL-S182.md). Dérivées/source identiques
après actualisation, admission, saturation/reprise, renouvellement et restauration.
S182-1 mesure le coût ; la portée physique de S181 reste inchangée.
