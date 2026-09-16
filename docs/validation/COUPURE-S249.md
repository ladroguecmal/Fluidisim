# Coupure spectrale de l'image — S249

## Protocole avant construction

Décision : [ADR-148](../adr/ADR-148-filtrage-spectral-image.md).
Consommateur : le rendu de l'hôte ; filtrage par défaut, témoin désactivable.

1. Tests analytiques des poids : conservation proche, zéro à Nyquist, monotonie,
   continuité ; chaque bande majore les modes qu'elle contient.
2. Projection : distances calculées depuis les mêmes voisins que le maillage ;
   référence, rasante, haute, 640×360 et 960×540.
3. GPU contre somme CPU des coefficients filtrés, sans grille CPU copiée du GPU :
   hauteur sous 3 mm (S201). Publier séparément écart au champ complet, pentes et
   quantité de modes non résolus conservés (doit être nulle).
4. Retour de caméra à instant fixe : même résultat GPU au bit sur cette machine.
   Rejouer aussi la réception existante sans filtre, sans modifier ses seuils.
5. Coût avec/sans sur scène multi-sources ; allocations de boucle, alimentation,
   format et techniques présents/absents publiés selon ADR-131/145.

Arrêt : intégration et critères ci-dessus éprouvés ; limites explicites pour impacts,
normales dans la transition, filtrage conservateur par bandes et seconde cible.

## Réception

`--multi --spectral-verify` : 36 cas (trois poses, deux formats, âges 3/12/16 s,
grille et somme directe). 558 sondes à 640×360, 828 à 960×540, rangées jusqu'au
bord lointain inclus. La passe compute reçoit les **indices de sommets** et reconstruit
sur GPU le point et ses huit voisins, comme la passe de rendu. Le CPU les reconstruit
indépendamment ; sa somme modale filtrée corrige la référence complète du cœur.
Les impacts restent ceux du cœur dans cet oracle : leur approximation reste donc testée.

- Pire erreur de hauteur : **0,339985 mm**, sous les **3 mm** de S201.
- Pires erreurs de pentes : **0,0009401 / 0,0002247**, publiées sans seuil perceptif.
- Pire écart volontaire au champ complet : **1,093772 m**, dans le lointain de B.
  Ce nombre n'est pas une erreur numérique et n'est pas comparé aux 3 mm.
- Retour hors champ puis retour à la pose, à instant fixe : **identique au bit** pour
  chaque composante GPU des 18 cas avec grille.
- Contre-épreuve GPU : un mode isolé par bande, phase nulle ; amplitude 1 proche,
  poids attendu dans la transition, **zéro exact** au-delà de Nyquist. Les tests CPU
  vérifient monotonie, raccords et majoration des modes par leur bande.
- `--multi --verify` sans filtre : 46 contrôles historiques et six contrôles de
  reconstruction/couture passent ; seuils conservés. La réception spectrale a été
  rejouée après le raccourci du chemin direct non filtré (`h = 0` donne poids 1).
- Cœur/harnais release hors ligne : **437 réussis, 16 ignorés** (bancs explicites).
  Hôte : **16 réussis, 1 ignoré**. Le premier lancement debug du cœur a été arrêté
  pour exécuter la suite entière en release ; aucune réussite debug globale revendiquée.

Le pas inclut ici les diagonales et les voisins dans les deux sens : il est plus
conservateur que la métrique S247. La transition commence **avant** Nyquist ; la zone
atténuée dépasse donc la seule bande rouge S248. Attendre d'avoir dépassé Nyquist
pour commencer une transition continue conserverait nécessairement des modes non résolus.

## Coût de la combinaison (ADR-131)

RTX 5070 Laptop, DX12, Windows, 960×540, maillage 481×271 ; scène S235, trois
sillages (4 096 modes communs), huit impacts échelonnés. Batterie `BatteryStatus=2`,
99 %, secteur constaté avant et après. 120 images après dix de mise en régime,
âges `âge0 + i/60`, i=0..129 ; aucun autre test du projet exécuté pendant ce banc.

Présents : phases repliées B, table d'impact, préparation temporelle, grille bicubique,
visibilité, mutualisation des sillages, **filtre B et huit bandes du sillage**.
Absents : FFT, LOD temporel, maillage adaptatif, parallélisme CPU persistant,
filtrage des impacts. Mémoire GPU réservée : neuf grilles de 16 384 vec4 f32,
**2,25 Mio**, soit **2 Mio supplémentaires**, sans allocation par image.

`--multi --spectral-bench`, deuxième passage, médianes en ms :

| pose / âge initial | GPU complet | GPU filtré | dont cuisson filtrée | CPU complet / filtré |
|---|---:|---:|---:|---:|
| référence / 3 s | 0,46125 | 1,22096 | 1,12797 | 3,9908 / 4,1092 |
| référence / 12 s | 0,48554 | 1,33274 | 1,23526 | 4,1726 / 4,2603 |
| rasante / 3 s | 0,46166 | 1,21680 | 1,12656 | 4,0246 / 4,0704 |
| rasante / 12 s | 0,48451 | 1,32842 | 1,23549 | 4,0657 / 4,1898 |

Le premier passage a mesuré **0,841 / 2,259 ms** à référence/3 s, puis est revenu
vers **0,462 / 1,219 ms** à rasante/3 s. Alimentation inchangée ; cause non attribuée.
C'est la raison du second passage, pas un résultat effacé. Le profil de 2 ms n'est
**pas garanti** : CPU toujours dominant, et dépassement GPU observé au premier passage.
La qualité coûte environ **0,76–0,85 ms GPU** en régime ; la cuisson des bandes porte
l'essentiel du surcoût. Ce n'est pas une optimisation de vitesse revendiquée.

Fenêtre rasante : intervalle médian **4,8311 ms** (207 Hz), CPU **4,0589 ms**, GPU eau
**1,2062 ms**. Sur 590 images : `update` **0 allocation**, pile **133 / 18 509 octets**
par image, constantes, comme ADR-145. Premiers relevés de fenêtre effectués pendant
la suite du cœur exclus de la comparaison de coût ; la comparaison retenue est le banc
hors écran à âges identiques ci-dessus.

## Limites et reproduction

Les amplitudes des modes non résolus sont nulles ; ce n'est pas un certificat d'absence
de tout alias dans une image : modulation spatiale des poids, reconstruction polynomiale,
reflets spéculaires et impacts non filtrés subsistent. Les bandes peuvent retirer des
modes déjà résolubles. Normales sans dérivée spatiale du filtre, pas de réception perceptive
en mouvement, pas de seconde cible. La coupure ne rend ni le rayon ni la durée de la recette
plus honnêtes : avertissement ADR-132 conservé.

Depuis la racine :

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --multi --spectral-verify
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --multi --spectral-bench
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --multi --cadence --rasant
```

Sorties locales conservées sous `viewer/captures/s249/` (non versionnées).
