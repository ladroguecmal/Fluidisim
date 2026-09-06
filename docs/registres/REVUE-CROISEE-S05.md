# Revue croisée des vingt ADR — S05

Confrontation systématique des ADR-001 à ADR-020 et des quatre spécifications, à la recherche de
**contradictions entre documents** — et non de défauts internes à chacun, déjà traités lors de leur
écriture.

Vingt décisions produites en quatre sessions n'avaient jamais été relues les unes contre les
autres. Douze écarts ont été trouvés, dont **deux de gravité 1**.

Gravité : **1** = deux documents s'excluent, une décision doit trancher · **2** = incohérence
opérationnelle, corrigeable par précision · **3** = documentaire ou mineur.

---

## Tableau

| # | Écart | Grav. | Résolution |
|---|---|---|---|
| R01 | L'échelle de `dx` ne suit pas le rapport annoncé | 3 | note corrective ADR-006 |
| R02 | L'aération présentée comme « exception à I-04 » sans en être une | 2 | **ADR-021** |
| R03 | Deux origines concurrentes pour une même onde répliquée | **1** | **ADR-021** |
| R04 | `domaines_max` contredit le budget de temps | **1** | note corrective ADR-012 |
| R05 | L'élagage des paquets W casse la cohérence des sillages | 2 | **ADR-021** |
| R06 | La dégradation la moins chère coûte du travail au pire moment | 2 | note corrective ADR-012 |
| R07 | La cellule de 64 m déclarée trop grossière par deux ADR séparés | 2 | note corrective ADR-006 |
| R08 | Les poches d'air n'ont pas de ligne dans la table d'autorité | 2 | **ADR-021** |
| R09 | Tuile FFT contre argument anti-pavage | 3 | critère d'acceptation à ajouter |
| R10 | Alignement 4096 m non déclaré | 3 | note, aucun changement |
| R11 | Son local contre latence réseau | 3 | renvoi croisé |
| R12 | `lambda_cut()` porté par le mauvais objet | 3 | correction SPEC-004 |

---

## R03 — Deux origines concurrentes pour une même onde répliquée *(gravité 1)*

**Constat.** ADR-005 §3 affirme que la transduction δ→W est « la seule voie par laquelle δ peut
influencer le monde répliqué », et ADR-009 §3 organise la validation serveur de cette émission.
Mais I-10 et ADR-009 §4 posent que **le serveur n'exécute jamais δ**. Le serveur ne peut donc pas
transduire, et deux clients qui transduisent produiront des paquets différents à partir de
solveurs non déterministes.

Une onde répliquée aurait ainsi deux origines incompatibles : le client par transduction, le
serveur par on ne sait quoi.

**Ce que la contradiction révèle.** Le mécanisme de validation d'ADR-009 §3 — plafonner une demande
client par une cause connue du serveur — supposait implicitement que le serveur connaissait déjà la
cause. S'il la connaît, il peut émettre l'événement lui-même, et le chemin client → serveur devient
inutile.

**Résolution → ADR-021.** Les ondes répliquées sont émises par le **serveur, depuis leurs causes**.
La transduction client ne produit que du `W_local`, cosmétique. Le chemin d'énergie
client → serveur disparaît, et avec lui l'angle mort A16 dans sa totalité.

**Argument de fermeture** — il fallait le vérifier avant d'accepter la résolution : un phénomène
purement δ pourrait-il mériter une onde répliquée ? Non, par construction : tout ce qui dépasse
`λ_cut` appartient à W et non à δ (ADR-001, ADR-005 §2.1). Aucun phénomène de conséquence gameplay
ne peut naître exclusivement dans δ.

## R04 — `domaines_max` contredit le budget de temps *(gravité 1)*

**Constat.** ADR-012 §3 fixe indépendamment `memoire_blocs = 384 Mo`, `domaines_max = 24` et
`gpu_sim_ms = 2,5`. SPEC-001 §2.4 vérifie la cohérence **mémoire** : 24 domaines de type bateau à
13,8 Mo tiennent dans 384 Mo. La cohérence **temporelle** n'a jamais été vérifiée.

24 domaines dans 2,5 ms, c'est **0,10 ms par domaine**. Pour les 432 000 cellules éparses d'un
domaine bateau à `dx = 0,10 m` (SPEC-001 §2.3), cela demande de l'ordre de **4·10⁹ mises à jour de
cellule par seconde de GPU**, pour un solveur à surface libre avec projection de pression. C'est
optimiste d'au moins un ordre de grandeur.

Le budget mémoire autorise donc environ six fois plus de domaines que le budget de temps ne peut en
servir.

**Ce que la contradiction révèle.** `domaines_max` n'est pas un paramètre : c'est un **résultat**.

```
domaines_max = min( mémoire_disponible / taille_domaine ,
                    budget_ms          / coût_domaine_ms )
```

Le second terme est inconnu jusqu'à B3. Le profil de qualité d'ADR-012 §3 est sur-spécifié : il
fixe une valeur dérivée comme si elle était libre, et la fixe de façon incompatible avec ses
propres autres valeurs.

**Résolution.** Note corrective dans ADR-012 : `domaines_max` est calculé à l'initialisation à
partir du coût mesuré par le solveur (`SolverCaps::cost_per_block_ms`, ADR-007 §2) et non déclaré.
Le profil ne porte que les deux budgets et la mémoire.

**Leçon associée.** Un profil de qualité qui contient à la fois des ressources et des capacités
dérivées finira toujours par se contredire. Ne déclarer que les ressources.

## R02 — L'aération présentée comme une exception à I-04 *(gravité 2)*

**Constat.** ADR-014 §5.2 écrit : « C'est une exception contrôlée à l'invariant I-04 », puis décrit
aussitôt quelque chose qui n'en est pas une — la part d'aération issue de W et du vent est
déterministe (donc autoritaire parce qu'elle vient de W, pas de δ), et la part issue de δ est
plafonnée et non autoritaire.

Le texte est juste ; l'étiquette est fausse.

**Pourquoi la gravité n'est pas 3.** Une exception admise à un invariant se cite, puis se
généralise. Six mois plus tard, quelqu'un invoquera « l'exception aération » pour justifier une
seconde entorse, et l'invariant central du système d'autorité aura été perdu par accident de
vocabulaire.

**Résolution → ADR-021.** Le champ d'aération se scinde explicitement en `A_rep` et `A_local`.
Aucune exception. L'invariant I-04 reçoit une phrase de clarification interdisant l'argument
d'exception.

## R05 — L'élagage des paquets W casse la cohérence des sillages *(gravité 2)*

**Constat.** ADR-014 §2.3 revendique qu'un sillage utilisé pour pister un navire est « cohérent
entre joueurs », parce que l'écume permanente est re-dérivée de W. ADR-012 §4 rang 6 autorise, sous
contrainte de budget, « réduire le nombre de paquets W (fusion des plus faibles) ».

Deux joueurs à des profils de qualité différents n'ont donc pas le même jeu de paquets, donc pas le
même sillage, alors qu'une conséquence gameplay — la détection — en dépend.

**Résolution → ADR-021.** L'élagage ne porte que sur `W_local` et sur les paquets `W_rep` dont
l'amplitude est passée sous le seuil de pertinence gameplay. Un paquet répliqué au-dessus du seuil
n'est jamais élagué, quel que soit le profil. Le rang 6 d'ADR-012 est amendé en conséquence.

## R06 — La dégradation la moins chère coûte du travail au pire moment *(gravité 2)*

**Constat.** ADR-012 §4 classe « rétrécir l'emprise des domaines non focaux » au rang 1, au motif
qu'ADR-005 §5 rend l'opération visuellement gratuite. Mais ADR-005 §5 la décrit comme
« transduction δ→W puis amortissement sur τ ≈ 0,3 s » : elle est *visuellement* négligeable, pas
*calculatoirement* gratuite.

Le système déclenche donc une mesure de flux, une agrégation par secteurs et une émission de
paquets **exactement à l'instant où il manque de budget**. La réaction à la surcharge aggrave
transitoirement la surcharge.

**Résolution.** Note corrective dans ADR-012 : sous contrainte de budget, le rétrécissement emprunte
un chemin dégradé — amortissement sans transduction, avec perte d'énergie assumée. Cela rejoint la
question laissée ouverte en ADR-005 §7.4 (« la transduction doit-elle volontairement perdre 10 à
20 % ? ») et lui donne un premier cas d'usage : **sous pression, elle perd tout**.

## R07 — La cellule de 64 m, trop grossière selon deux ADR indépendants *(gravité 2)*

**Constat.** ADR-017 §7.2 note que la cellule `HydroGrid` de 64 m est trop grossière pour une
rupture de glace crédible et propose « une subdivision dédiée, 2 à 4 m ». ADR-018 §7.2 note qu'elle
est trop grossière pour un gué et propose « une publication à la sous-cellule le long des rivières
et des rivages ».

Deux ADR écrits à une heure d'intervalle inventent séparément la même parade. Ce n'est pas une
contradiction : c'est un **concept manquant** dans ADR-006, que chacun réinvente à sa façon — et
donc deux implémentations divergentes garanties.

**Résolution.** Note dans ADR-006 : la publication à la sous-cellule est un mécanisme unique de la
`HydroGrid`, avec un facteur de subdivision par type de donnée, et non une invention locale par
consommateur.

## R08 — Les poches d'air sans autorité déclarée *(gravité 2)*

**Constat.** La table d'autorité d'ADR-008 §1 énumère B+W, W, V et δ. Les poches d'air d'ADR-015,
écrites une session plus tard, portent une flottabilité déterminante — une coque retournée flotte
grâce à elles — et n'apparaissent dans aucune ligne.

**Résolution → ADR-021.** Elles sont calculées par une équation d'état à partir de l'état de V et de
la pose du solide, tous deux répliqués : elles sont donc **autoritaires**, et rejoignent la ligne V.

## R09 — Tuile FFT contre argument anti-pavage *(gravité 3)*

ADR-004 §3 démontre qu'un océan pavé par régions produit des artefacts de raccord. ADR-004 §6.2
propose ensuite une tuile FFT pour le détail haute fréquence.

Ce n'est pas contradictoire — le défaut d'un pavage FFT est une **répétition** visible, pas une
bande de calme, et il ne porte aucune donnée gameplay. Mais l'acceptabilité n'a pas de critère.

**Action.** Ajouter au banc B1 : distance à partir de laquelle la répétition de la tuile devient
perceptible, en fonction de sa taille. Sans ce chiffre, la taille de tuile sera choisie à l'œil.

## R10 — Alignement 4096 m non déclaré *(gravité 3)*

Le seuil de rebasage d'ADR-002 §2.3 (4096 m) et le troisième niveau de la `HydroGrid` d'ADR-006 §2
(4096 m) coïncident. C'est heureux — une région de rebasage vaut exactement une cellule de niveau 3
— mais la coïncidence n'est déclarée nulle part, donc rien ne l'empêche d'être rompue par un
ajustement ultérieur de l'un des deux.

**Action.** Le déclarer comme alignement délibéré dans ADR-006 §2.

## R11 — Son local contre latence réseau *(gravité 3)*

ADR-016 §3 impose un retard acoustique (0,29 s à 100 m). ADR-009 §7.2 laisse ouverte la question de
l'anticipation locale d'un événement causé par le joueur. Un événement proche, s'il attendait
l'aller-retour serveur, devrait être joué avant d'être reçu.

Pas une contradiction, mais une dépendance non signalée : **la réponse à la question ouverte
d'ADR-009 §7.2 conditionne la faisabilité d'ADR-016 §3**. Renvoi croisé ajouté.

## R12 — `lambda_cut()` porté par le mauvais objet *(gravité 3)*

SPEC-004 §6 place `lambda_cut()` sur `IBackgroundField`. Or `λ_cut` est la frontière entre les
couches W et δ : elle appartient à la configuration du système, pas au champ de fond, qui n'a
aucune raison de la connaître.

**Action.** Déplacer vers `WaterConfig`, et la passer au solveur par `DomainConfig`.

---

## R01 — L'échelle de `dx` ne suit pas le rapport annoncé *(gravité 3)*

ADR-006 §3.2 énonce `{0,02 ; 0,05 ; 0,10 ; 0,25 ; 0,50 ; 1,00} m` — « six niveaux, rapport ≈2,5 » —
puis justifie le choix de 2,5 plutôt que 2. Les rapports réels sont **2,5 · 2 · 2,5 · 2 · 2**.

C'est en réalité une série 1–2,5–5 par décade, choix classique et défendable, mais ce n'est pas ce
que le texte prétend. Note corrective.

---

## Ce qui a été vérifié et tient

Un audit qui ne rapporte que des défauts n'est pas vérifiable. Contrôles passés sans écart :

- **Chaîne des budgets temporels** — tick 30 Hz (ADR-012 §7), V à 10 Hz (ADR-010 §4), publication
  de traversabilité à 5 Hz (ADR-018 §6) : diviseurs cohérents.
- **Tables numériques dupliquées** — dispersion, sillage, CFL, coût en `dx⁻⁴` : SPEC-001 et les ADR
  qui les citent concordent, y compris les valeurs dérivées (`2,5⁴ = 39`).
- **Cohérence mémoire** de SPEC-001 §2.4 avec ADR-012 §3 : exacte (le défaut est ailleurs, R04).
- **Plafond de la pose de rendu** (8 cm, 3°) : identique dans ADR-008 §1 et SPEC-004 §7.2.
- **Chaîne de déterminisme** — ADR-003 (arithmétique), SPEC-003 §2 (régimes D1/D2/D3),
  SPEC-004 §8.2 (`parallel_reduce_ordered`) : cohérente de bout en bout.
- **Invariants I-01 à I-14** contre les vingt ADR : seul I-04 était contredit (R02, R03), les
  treize autres tiennent.
- **Traçabilité** des 30 sections sources : aucune section perdue entre S01 et S04.

---

## Suite

| Action | Où | Statut |
|---|---|---|
| ADR-021 — autorité des grandeurs dérivées | R02, R03, R05, R08 | écrit |
| Note corrective `domaines_max` | ADR-012 §3 | appliquée |
| Note corrective dégradation rang 1 et rang 6 | ADR-012 §4 | appliquée |
| Note corrective échelle de `dx` et sous-cellule | ADR-006 | appliquée |
| Renvoi ADR-005 §3 → ADR-021 | ADR-005 | appliqué |
| Reformulation aération | ADR-014 §5.2 | appliquée |
| Clarification I-04 | `01_INVARIANTS.md` | appliquée |
| Critère de répétition de tuile FFT | B1 | **appliquée en S05** *(statut corrigé en S15 : il annonçait « à ajouter » depuis dix sessions alors que `PLAN-BENCHMARK` B1 porte l'ajout)* |
| `lambda_cut()` vers `WaterConfig` | SPEC-004 | appliqué |
