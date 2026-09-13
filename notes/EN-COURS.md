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

Session : S222 — en cours
Agent : Claude Code, Opus 5 (fichiers, git et cargo disponibles)
Entrée : « Reprends le projet », conversation neuve — **aucune mémoire de S217 à S221**, tout a été
relu depuis `AGENTS.md`, `REPRISE.md` §4 et les trois ADR 135/136/137. master et trois copies à
c33953a, jeton libre, maillons 0. Copie principale.
Objectif : **A254, la part somme** — ce que coûte une scène à plusieurs sillages dans le budget de
pente, et ce qu'une borne locale conjointe lui rendrait.

### Ce que la relecture établit avant toute mesure

`mixed_water::slope_floor(impacts, pressure, time)` somme **un majorant par impact**
(`slope_max_at`, ADR-133) **plus un seul terme de pression** — la signature ne prend qu'un
`Option<&bound_pressure::Prepared>`. Donc :

- **plusieurs sillages ne peuvent entrer dans le budget qu'en partageant un journal**, une recette
  et une emprise ; c'est cette configuration que la ligne de suite nomme, et c'est la seule que le
  cœur sache composer aujourd'hui ;
- dans cette configuration, les sources partagent les **emplacements** du demi-spectre : leurs
  amplitudes modales s'additionnent **en complexe**, donc le terme de pression est déjà
  sous-additif par construction. La « part somme » d'A254 ne se lit donc pas sur le nombre de
  sillages de la même façon que sur le nombre d'impacts, et il faut le mesurer avant de le dire.

Les bornes locales (ADR-135, 136, 137) et leur partition (S219) **ne sont branchées sur aucune
admission** : elles publient, elles ne refusent pas. La question de la session est de savoir si
elles ont de quoi le faire.

### Thèse et critères, déclarés avant toute mesure

1. **Ce que l'enveloppe globale fait du nombre de sources**, mesuré et non supposé : à deux et
   trois sillages, proches puis éloignés, le rapport de `slope_envelope()` à sa valeur pour une
   seule source. Si la composition complexe la rend franchement sous-additive, A254 n'a pas de
   « part somme » du côté des sillages, et c'est la réponse.
2. **Ce qu'une borne locale conjointe rendrait** : partition spectrale (ADR-137) sur l'emprise, à
   **budget d'évaluations égal** d'une configuration à l'autre, contre le **maximum réel**
   échantillonné finement. Gain, et ce qui reste au-dessus du maximum.
3. **Traduire en part de π/7** : le budget d'admission complet — impacts toujours sommés
   (ADR-133) plus le terme de pression — avec l'enveloppe actuelle puis avec la borne partitionnée.
   Dire combien de sources passent dans chaque cas. C'est la seule forme qui réponde à A254.
4. **Le coût de passe fait partie du verdict.** S219 mesure 18 à 36 s pour 32 767 évaluations : un
   gain réel peut être **inutilisable** comme terme d'admission par image. Le dire, plutôt que
   publier un gain sans son prix.
5. Aucun seuil de réussite présumé ; publication avec techniques, domaine et rang de passage
   (L289). Aucune admission migrée avant P6.

**Prédiction écrite pour être contredite** : éloignés, deux et trois sillages donnent une enveloppe
globale qui croît nettement moins vite que le nombre de sources (facteur ≤ 1,6 à trois), parce que
les phases modales se désalignent ; le maximum réel, lui, reste proche de celui d'une source.
La borne locale partitionnée rend alors l'essentiel de l'écart — moins d'un facteur 1,5 au-dessus
du maximum — mais **à un coût de plusieurs secondes**, donc sans usage possible comme terme
d'admission par image. Autrement dit : je prédis un gain réel et inutilisable en l'état, et c'est
ce résultat-là qu'il faut savoir écrire s'il se produit.

### Plan

- [x] **P1** — jeton, entrée, ce que la relecture établit, thèse, critères, prédiction, plan seuls.
- [ ] **P2** — fixture multi-sillages : deux et trois sources dans un même journal, proches puis éloignées ; témoin que la composition est bien partagée par emplacement, et refus éventuels de la bibliothèque.
- [ ] **P3** — enveloppe globale et maximum réel par configuration ; sur-additivité mesurée.
- [ ] **P4** — borne locale conjointe : partition spectrale à budget d'évaluations égal ; gain, reste au-dessus du maximum, **coût de passe**.
- [ ] **P5** — part de π/7 : budget d'admission complet avec les impacts sommés ; combien de sources passent, avant et après.
- [ ] **P6** — décider : ADR si quelque chose est rendu **et** utilisable ; sinon constat motivé, et retour à la file (cadence complète de l'hôte, V-noyau).
- [ ] **P7** — document de réception (en-tête ADR-131 D3) ; suite complète `code/`.
- [ ] **P8** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide : le travail commence en P2)*
