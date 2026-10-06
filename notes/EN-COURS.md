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

Session : S513 — **en cours**. En autonomie, **2.6 (niveaux C0 et C2 d'ADR-011) et 6.2, le courant** : 6.2 dit « Manquent le courant, la
turbulence » ; le courant n'existe nulle part dans l'eau (2.6, absente, conçue par ADR-011). Et le lot des registres (dû).

**Ce que la session fait.** `CurrentWater` : une requête qui enveloppe une autre (B, ou B + W) et lui ajoute un courant C0 (vecteur de
surface constant) et son profil C2 (`u(z) = u_fond + (u_surface − u_fond)·exp(z/D)`) : la vitesse de l'eau augmentée du profil, et le champ
de vagues **advecté** par le courant de surface (`η(x, t) = η₀(x − U·t, t)`, Galilée : l'effet Doppler d'un courant uniforme). Le champ C0/C2
en lecture seule (ADR-011 §2). Un corps qui traîne dérive avec lui.

**Ordre de grandeur, écrit avant.** Une houle de 6 s (λ = 56 m, c = 9,37 m/s) sur un courant de 1 m/s : les crêtes vont à 10,37 m/s,
+10,7 %. La dérive d'un pavé traîné (`C_d` = 1) : la vitesse relative `w(t) = w₀/(1 + k·w₀·t)`, `k = ½ρ·C_d·A/m` — pour la bouée de 0,5 m
à 500 kg/m³ (A ≈ 0,1 m² immergé de face, m = 50 kg), `k` ≈ 1 m⁻¹ : la moitié de l'écart en ≈ 1 s.

**Critères, écrits avant.** (1) sans courant, la requête rend l'enveloppée au bit ; (2) la vitesse d'une crête de houle sous courant :
`c + U` à 0,5 % ; (3) le profil vertical exact (à 10⁻¹²) ; (4) la dérive d'un pavé traîné en eau calme sous courant : `w(t)` à 1 % de
l'analytique. 2.6 passe à partiel ; 6.2 garde la turbulence pour seul manque.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — `CurrentWater` ; essais (1)–(4).
- [ ] **P3** — preuve ; liste 2.6 et 6.2 ; lot des registres ; rituel.

### Notes de reprise
