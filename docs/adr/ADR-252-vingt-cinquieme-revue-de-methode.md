# ADR-252 — Vingt-cinquième revue de méthode (S601–S605)

- **Statut : actée**, S606, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-251](ADR-251-vingt-quatrieme-revue-de-methode.md).

## 1. Les frictions relues, et leur suite

| friction | coût | suite |
|---|---|---|
| **La taille de la suite écrite par incrément** : de preuve en preuve, « suite du cœur : N » valait la précédente plus les essais ajoutés, sans mesure. En S602, 777 ; S603 l'a mesurée — **793 essais listés** (dont 17 ignorés, les longs : 776 exécutés). Deux bases mêlées (listés ou exécutés) et une dérive d'un dans la chaîne | un nombre faux dans une preuve, découvert par hasard | **protection élargie** (D1) |
| Des nombres du plan faux avant la mesure — la profondeur minimale d'un support estimée à la main (S603 : 107,5 m, en réalité 107,14), une chute « ×101 » mal comptée et un terrain qui donnait deux conflits au lieu d'un (S604) — **arrêtés par les assertions du script du plan** avant tout commit | aucun | **rien à changer** : ADR-243 D1 et ADR-251 D1 ont tenu |
| Une garde définie trop large (S603 : « une plage change » comptait un rayon qui franchit une borne de 6 cm) — arrêtée par le script du plan, redéfinie avant la mesure (un décalage, non un franchissement) | aucun | rien à changer |
| Un script d'édition qui échoue, suivi dans la même commande d'un script qui s'exécute sur l'état non édité (S604 : un saut de ligne au lieu de `&&`) — vu à la sortie, défait par `git checkout` | une relance | rien de nouveau : les commandes enchaînées se lient par `&&` (déjà la règle des commits, ADR-242) |
| Le lot dû en S605 non annoncé par la suite écrite en S604 ; le rituel l'a rappelé (code 3, ADR-242) | une relance | rien à changer |

## 2. Décision

**D1 — Un nombre de la preuve se mesure dans la session, par une commande nommée, jamais par incrément.** La taille de la suite du cœur :
`cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib -- --list`, les lignes « : test » — **les essais
listés, ignorés compris** (796 en S605). Les preuves antérieures à S603 portent l'ancienne base (exécutés, par incrément) ; elles ne sont
pas réécrites (élargit ADR-243 D1 de la feuille du plan à la preuve).

## 3. La prochaine revue

S611.
