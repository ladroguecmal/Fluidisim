# La vague qui plonge dans le relais, moment par moment — analyse (S690)

*S690, 2026-10-08, à la demande de l'utilisateur : « analyse pour chaque moment distinct de la simulation, cherche les améliorations et les
blocages par moment, puis en vue du global ; puis réfléchis à la façon de gérer le contexte d'une discussion ».*

**Le montage** : la vague de S647 (`H` = 0,15 m sur 0,5 m d'eau, pente 1:12, plongeante), APIC 3D à 2,5 cm sur 10,775 m (236 848
particules), Saint-Venant 2D au-delà, 16 fils. **Les mesures** viennent de la progression de l'essai accéléré, avant le remède du raccord,
et du diagnostic de l'effondrement du pas.

## 1. Les moments

| moment | temps simulé | le pas | le coût (horloge par seconde simulée) | ce qui s'y passe |
|---|---|---|---|---|
| 0. la mise en place | — | — | quelques secondes | le fond, les fractions des faces, 237 000 particules posées |
| 1. la propagation au large | 0 → 1,5 s | 10 ms (le plafond) | ≈ 105 s/s | l'onde traverse 5 m de fond plat et le pied de pente |
| 2. la levée | 1,5 → 2,6 s | 9 → 5 ms | 130 à 165 s/s | l'onde se raidit sur la pente |
| 3. le retournement, le jet, l'air | 2,6 → 2,9 s | 5 → 3,5 ms | ≈ 260 s/s | la crête se retourne (2,64 s), le jet retombe, l'air s'enferme (2,82 s) |
| 4. le ressaut au raccord | 2,95 → 3,0 s | **3,5 ms → 0,1 ms** | **≈ 660 à 1 500 s/s** | la masse déferlée atteint le raccord ; la dernière colonne 3D se vide |
| 5. le reflux | 3,0 → 4 s | 0,2 à 0,7 ms (avant remède) | des heures | l'eau redescend, repasse dans la 3D, oscille |

## 2. Moment par moment : les blocages, les améliorations

**0. La mise en place.** Aucun blocage. Les fractions des faces sont échantillonnées 16 × 16 par face : sans importance à cette taille.

**1. La propagation au large — la 3D n'y sert à rien.**
- **Blocage** : 75 % des particules portent une onde qui ne se retourne pas. Elles coûtent 2 minutes et demie avant que quoi que ce soit
  d'utile n'arrive.
- **Le pas est plafonné à 10 ms par mon appel**, non par la stabilité.
- **Améliorations** :
  - le relais au large (S650 : Saint-Venant ou B jusqu'au pied de la zone de déferlement), avec la 3D réduite à la bande qui se
    retourne : **÷ 5 particules** ici. Il demande la zone de colonnes avec la sortie à droite, refusées ensemble aujourd'hui (S682) ;
  - la bande étroite en profondeur (S413 : les particules seulement près de la surface, ÷ 5 à 7 au même résultat) ;
  - le plafond du pas relevé là où la stabilité le permet.

**2. La levée.** Le pas baisse avec la vitesse, et c'est normal. Les mêmes améliorations que le moment 1 s'appliquent. Dans la 3D, la
projection reste la part la moins parallèle (S483 : ×1,3 seulement) ; un ensemble de fils persistant la ferait gagner.

**3. Le retournement, le jet, l'air — la seule part qui demande la 3D.** Aucun blocage mesuré : le pas descend à 3,5 ms, comme dans le
tout-3D. C'est ici que la maille fine et les fils doivent aller. L'amélioration est un raffinement local, la 3D fine seulement autour du
jet.

**4. Le ressaut au raccord — le blocage de S690.**
- **Mesuré par le diagnostic** :
  - à 2,98 s, la masse déferlée passe dans Saint-Venant et la dernière colonne 3D se vide (de 4 particules par rangée à 0) ;
  - le niveau lu tombe alors au fond, et la hauteur à son plancher de 1 mm ;
  - la vitesse imposée au bord monte à **1 644 m/s**, et les particules posées partent à **−138 m/s** ;
  - une éclaboussure soulève aussi le niveau lu (0,62 m au lieu de 0,50) ;
  - le pas tombe à 0,1 ms : c'était **le calcul de 8 h**.
- **Remède (en essai)** :
  - le plancher de la hauteur porté à un quart de maille ;
  - les vitesses du bord bornées par la célérité `|u| + 2·√(g·h)` ;
  - le niveau borné par le volume de la colonne.
- **Plus loin** :
  - lire le bord sur deux colonnes, et non une ;
  - un raccord qui recule avec la vague : la bande 3D qui vit avec elle (S637–S638).

**5. Le reflux.**
- **Blocage, avant le remède** : les oscillations du raccord vide entretenaient le pas minuscule.
- **Mesuré en S689** : la sortie de la 3D passe à 79 % par le remboursement de la dette, et non par l'advection. Les particules du bord
  n'avancent pas à la vitesse que le flux demande.
- **Amélioration** : la vitesse des particules de la dernière colonne rapprochée de celle de la face ouverte (le transfert grille →
  particule à la frontière).

## 3. Le global

**Le coût.**
- Avant le remède : environ 23 min pour 4 s, soit 350 fois plus lent que le temps réel.
- Pour un jeu, la 3D ne doit vivre que là où l'eau se retourne et quand elle se retourne. Les gains se multiplient :

| levier | gain attendu | état |
|---|---|---|
| le raccord sans effondrement du pas (moment 4) | des heures → minutes | en essai |
| la 3D réduite à la bande de déferlement (relais au large + au rivage) | ÷ 5 | les deux raccords existent, pas ensemble |
| la bande étroite en profondeur (S413) | ÷ 5 à 7 | dans APIC, pas avec le fond lisse ni les raccords |
| la bande qui naît et meurt avec la vague (S637–S638) | la 3D absente hors des vagues | conçue |
| le plafond du pas, les fils persistants pour la projection | × 1,5 à 2 | simple |
| la carte (GPU) pour APIC | × 10 à 50 | δ a sa carte ; APIC 3D non |

**Les blocages de méthode**, révélés par S690 :
- une durée estimée sans mesurer le coût d'un pas ;
- un seul cœur ;
- aucune progression affichée ;
- un plancher numérique (1 mm) au lieu d'une borne physique.

Ils entrent dans la revue S691.

## 4. Le contexte d'une discussion : large, ciblé, sans inutile

**Ce qui consomme le contexte sans rien apporter** :
- les sorties longues (la liste des essais de `cargo`, les avertissements répétés) ;
- les fichiers relus en entier ;
- les longues commandes en ligne qui échouent et se relancent ;
- les vérifications d'attente répétées ;
- les tableaux recopiés deux fois (dans la réponse et dans la preuve).

**Ce qui garde le contexte large sans le charger** :
1. **La mémoire hors de la discussion.** Le dépôt porte la connaissance durable : la BOUSSOLE, REPRISE, EN-COURS, les preuves, le
   journal. La mémoire de l'agent porte les leçons transversales. Une discussion compactée ne perd alors que ce qui est déjà écrit.
   **Règle** : rien d'important ne vit seulement dans la discussion. Une mesure, une décision ou une cause trouvée s'écrit dans le dépôt
   dans la minute.
2. **Des sorties filtrées.** Seules les lignes marquées (`S690 :`, `progression`) sont relues. Un essai écrit ses nombres en une ligne
   qui porte son nom de session.
3. **Lire par morceaux.** `grep` d'abord, puis la plage utile ; jamais un fichier entier de 2 000 lignes.
4. **Les scripts dans des fichiers** (ADR-267 D2), lancés en une ligne ; leurs nombres en sortie, non leur texte.
5. **Une fiche de campagne** (la conception du relais, RELAIS-RIVAGE-S679) : l'état de la campagne en une page, que la discussion relit
   au lieu de reconstituer l'historique.
6. **L'attente notifiée.** Le calcul long notifie sa fin ; les relevés de progression ne se lisent que lorsqu'ils servent une décision.
7. **Les réponses courtes à l'utilisateur.** Les chiffres qui décident ; le détail est dans la preuve, où il est lié.

**Proposé (à décider)** :
- un outil `outils/essai.py` qui lance un essai depuis une copie du binaire, avec les fils, la progression et le filtre des lignes
  marquées, et qui rend un résumé de dix lignes ;
- une fiche de campagne pour chaque campagne longue.
