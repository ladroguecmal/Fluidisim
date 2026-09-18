# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S275 — terminée
Agent : Claude Opus 5, Claude Code desktop ; fichiers, git, cargo, Python, GPU local.
Entrée : suite de S274 dans la même conversation (consigne utilisateur : continuer sans attendre,
solliciter pour un jugement visuel). master d0ab484, copie unique.
Objectif : rendre δ visible pour la première fois — un domaine couplé sous une houle à crêtes
longues, rejoué dans `viewer/`, B seul contre B+δ, au pas de référence et au pas d'image — et
soumettre des captures à l'utilisateur (liste 8.7).

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — ADR-168 et protocole écrits avant le code : scène `--delta` (houle unidirectionnelle),
  domaine, pas de temps, bande extrudée le long des crêtes et fondus, ce qui n'est pas revendiqué,
  critères de fonctionnement et questions de revue.
- [x] **P3** — précalcul de δ dans l'afficheur (module `delta.rs`) : échantillons de B de la scène,
  pas couplé mobile au pas de référence et au pas d'image ; essais (échantillons plans, pas reçus,
  écart entre les deux pas en hauteur).
- [x] **P4** — couche δ côté GPU : tampon et paramètres, lecture Hermite le long de x, fondus,
  bascule clavier ; vérification GPU contre CPU de la couche.
- [x] **P5** — captures de revue (poses de jeu, trois variantes) et demande de revue à l'utilisateur.
- [x] **P6** — rituel §6.

### Notes de reprise

Pas couplé mobile ≈ 49 ms à 16 384 mailles (S274) : le précalcul est hors budget et déclaré tel.
δ x-z exige des échantillons plans : houle à `spread_turns = 0`. Similitude de Froude : le pas
d'image à λ = 100 m équivaut en `ωdt` à ≈ 3,5 ms sur le banc de 4 m.

P3+P4 (un commit : code imbriqué dans les mêmes fichiers) : rejeux reçus, écart pas d'image
hors éponge 0,25/1,33 mm ; GPU/CPU 7e-8 ; empreintes S254 identiques. Incident : `--multi --revue`
sans suffixe a réécrit les PPM R1 de captures/s254 (PNG envoyés intacts, non versionnés).
Un `sed` global avait touché d'autres lignes de main.rs : fichier restauré puis réédité.

P5 : 12 captures + 3 diagnostics (captures/s275), pixels B/B+δ 10–24 % > 4 niveaux (0 % vue
haute), 4 ms/16 ms 0 %. η' groupé (−47 mm à 20 s vers x = 0–32 m). Revue R10 envoyée, en attente.
