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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S354 — **en cours**. **Lot 5, A316 : ce qui dissipe au raccord particules ↔ colonnes** ; alternance
d'ADR-184 D1, après S353 (la v1 en scène vivante).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — *« Reprends le projet »*. S327 a laissé A316 attribué en partie : le meilleur montage (ensemencement
continu, échange en paroi) écarte la surface de 0,24 maille et amortit **4,2 % par période** à 5 cm, contre −0,4 %
pour APIC seul. Il en concluait que **les colonnes dissipent d'elles-mêmes** : 1,3 % avec 95 % du bassin en
colonnes, frontière près du mur.

**Un doute sur l'instrument, trouvé en préparant.** La jauge du ballottement compte les particules du quart gauche
du bassin. Avec 95 % en colonnes, ce quart est en colonnes, dont les particules sont **réensemencées** à chaque pas,
deux par rangée, `round(2h/dx)` rangées : la masse que la jauge y lit est quantifiée par demi-maille — 2,5 cm à
5 cm, pour une onde de 2 cm. La hauteur vraie des colonnes est `h`, exacte. Le 1,3 % a pu être lu sur un escalier.

Critères, écrits avant le code :
1. **L'instrument** : une jauge qui lit `h` dans la zone des colonnes et les particules ailleurs
   (`RACCORD_JAUGE=hauteurs`) ; colonnes seules possibles (`RACCORD_ZONE=1`, sans frontière). Au repos, colonnes
   seules : jauge constante à l'arrondi. Sans variable, tout au bit — les valeurs de S327.
2. **L'attribution** : ballottement, colonnes seules et 95 % en colonnes, à 5 et 2,5 cm, les deux jauges.
   **Prédiction** : avec la nouvelle, les colonnes seules amortissent comme APIC seul à 1 point près — elles ne
   dissipent pas, et le 4,2 % est à la frontière. Sinon, elles dissipent, et on éteint un à un les termes qu'APIC
   n'a pas.
3. **Selon 2** : la cause désignée — frontière ou colonnes — éprouvée seule, avec son témoin ; critères de S327
   inchangés : amortissement à 1 point d'APIC seul, période à 1 %, écart à la frontière < 0,5 maille.
4. Preuve §13, A316, file, liste.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la jauge des hauteurs, les colonnes seules ; critère 1.
- [ ] **P3** — l'attribution : colonnes seules et 95 %, deux mailles, deux jauges ; critère 2.
- [ ] **P4** — la cause que P3 désigne, éprouvée seule ; critère 3.
- [ ] **P5** — preuve §13, A316, file, liste ; critère 4.
- [ ] **P6** — rituel.

### Notes de reprise
- **P2.** `RACCORD_JAUGE=hauteurs` : particules libres comme `Apic::mesure`, colonnes par `h` au prorata de leur
  largeur dans la bande, attente nette si la frontière y tombe ; `RACCORD_ZONE=1` sans frontière (gardes de
  `ecart_frontiere` et de la série). **Sans variable, au bit** : écart 0,2436, amortissement 4,168 %, période aux
  zéros 2,12419 s (S327). **Critère 1, tel qu'écrit, manqué** : au repos, colonnes seules, la jauge des hauteurs
  varie de **0,126 mm** — le repos n'est pas immobile, 4,1 mm/s, comme APIC seul (0,110 mm, 4,4 mm/s). L'ancienne
  jauge y reste **à 0,5 exactement** : aveugle sous son palier. L'instrument suit l'eau ; la réponse « constante »
  supposée n'était pas celle du modèle.

