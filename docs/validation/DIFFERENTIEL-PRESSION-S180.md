# Différentiel de pression forcée — S180

S179-1/A50, fournisseur dans spectral_pressure::Field. Contrat et dérivation :
[ADR-116](../adr/ADR-116-differentiel-de-pression-forcee.md).

## Protocole avant code

- Mode fixe oblique : solution forcée fermée f64, toutes les dérivées en profondeur.
- Démarrage : eta=u=0, pression imposée et accélération non nulles ; contre-épreuve.
- Extinction semi-ouverte : pression imposée disparue, eta/u continus, accélération
  évaluée sur la branche libre. Ne pas différencier au centre d'une commutation.
- Deux modes et source mobile : différences finies spatiales/temporelles et résidu
  reçu via gradient de Bernoulli, limites d'arrondi explicites.
- Surface historique inchangée ; refus et sortie atomique ; préparation incrémentale
  et liaison FieldState conservent les nouvelles métadonnées physiques.
- Workspace et C02/C18 reçus ; aucun budget multiplateforme ou coût promis.

## Résultats

Les cinq nouveaux tests passent en debug et release.

- Mode oblique fixe : solution indépendante f64 à cinq instants, dont démarrage et
  extinction exacte, en profondeur. Budgets : 1e-7 sur la cinématique et son gradient,
  2e-7 sur l'accélération, 0,002 Pa ou Pa/m sur la pression profonde, 2e-5 sur
  la pression imposée et son gradient. La branche libre part de l'état forcé à 2 s.
- À la naissance, eta=u=0 mais p_dyn=56 Pa et du_dt_z=-56/1025 m/s². La source
  linéaire s'annule ; neutraliser son gradient de pression donne un résidu >0,05 m/s².
  Cette contre-épreuve est dans le test, sans modification du code de production.
  À l'extinction, pression imposée nulle, eta/u continus à 1e-7 et saut d'accélération
  reçu à 1e-6. Aucune différence centrale ne traverse la commutation.
- Deux modes croisés, source mobile : différences spatiales à 0,01/0,005 m,
  temporelles à ±1 ms. Budgets 2e-6 pour grad_u et Laplacien, 4e-6 pour du_dt,
  0,003 Pa/m pour grad_p_dyn et 0,004 Pa/m pour le gradient imposé. Source totale
  reçue via le gradient de Bernoulli à 6e-6 m/s² ; les résidus isolés diffèrent
  du total de plus de 1e-6 m/s², ce qui détecte l'oubli des termes croisés.
- À z=0, eta, u et grad_eta conservent les bits de sample. Refus tardifs, capacités,
  lot vide et préfixe publié reçus ; sortie intacte sur erreur, scratch modifiable.
  Préparation directe/incrémentale et FieldState/bind identiques. Métadonnées invalides
  et non-finis refusés. Slot passe de 48 à 64 octets ; pools dimensionnés par size_of.

Workspace reçu : 324 tests réussis, cinq ignorés (231+93 réussis, 2+3 ignorés).
C18 hash0x85c8bc610f551d11 et C02 hash0x0a3a3bcc945db263 inchangés, zéro échec.
Le compteur historique alloc_post_seal=1 des scénarios ne mesure pas ce nouveau
fournisseur. Son absence d'allocation est vérifiée par lecture du chemin, sans mesure
de coût ni certification multiplateforme.

Commandes depuis code/ :

```text
cargo test -p water-core spectral_pressure::differential
cargo test -p water-core spectral_pressure::differential --release
cargo test --workspace
cargo run -p water-harness --release -- check scenarios/C18-invariants.toml scenarios/C02-dispersion.toml
```

## Limites et suite

S179-1 réalisée pour le champ spectral préparé à un instant donné. Une réception sur
modes fournis n'est pas une réception de toutes les cuissons gaussiennes, de la
bathymétrie ou d'une pression de coque calibrée. A50/B4 restent partiels.

**S180-1, priorité S181 :** composition différentielle B+impacts+pressions dans un
contexte physique et temporel commun, pression appliquée comptée une fois, source
contractée après sommation et réception des interactions. Puis exposition monde et
cycle contrôleur ; coût, surface libre non linéaire et δ3D restent ouverts.
