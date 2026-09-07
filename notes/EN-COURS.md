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

```
Session          : S69
État             : terminée
Agent            : Claude Code (Opus 5 ; git et cargo disponibles)
Objectif         : Bilan d'avancement — taux de progression, goulot réel, étapes futures.
                   Demandé par l'utilisateur ; S64-2 est décalée à S70.
```

### Plan

- [x] **P1** — passation, jeton, plan seul.
- [x] **P2** — établir le bilan sur des compteurs vérifiés, pas sur les colonnes d'état des vieux tableaux (**A185**).
- [x] **P3** — rituel : journal, index, jeton, **fusion dans master**.

### Notes de reprise

Départ e839c77, master et worktree confondus. Les sessions S65 à S68 ont été conduites par Codex
et sont fusionnées ; rien n'était en cours.

**Une précaution de méthode, et elle vient d'A185.** Les colonnes « État » des tableaux d'actions
antérieurs à S45 ne sont pas fiables : plusieurs actions y sont marquées « ouverte » alors que des
notes en prose sous les tableaux les closent (S35-1 et S35-2 closes par une note de S36, par
exemple). **Ne pas compter les actions ouvertes à partir de ces colonnes.** Le bilan s'appuie sur
ce qui se vérifie : le nombre de fichiers, les compteurs de l'index tenus à jour par le rituel, la
sortie du harnais, et le chemin critique de `REPRISE.md` §4.

P2/P3 : BILAN-S69 écrit et indexé, entrée de journal, jeton rendu. Aucun code, aucun calcul.
Trois recommandations remontées : lancer B1 (seul banc exécutable, jamais lancé), trancher S63-1
(seul verrou de B2), et **décider si le projet passe à la construction** — ce dernier point hors
de portée d une session.
