# Flux du fond aux frontières — S270

## Critères déclarés avant correction

Consommateur : `Volume::step_perturbation_mobile`, ADR-165. Arrêt du lot : défaut
stationnaire reproduit puis corrigé dans ce pas, flux signés et bilan reçus,
refus/reprise et absence d'allocation vérifiés. La houle progressive complète
reste une réception séparée ; ne pas appeler ce cas constant une onde progressive.

1. Courant uniforme U=±0,5 m/s et niveau de fond a=±0,125 m, profondeur 2 m,
   domaine 4 m, dx=0,25 puis 0,125, dt=1 ms, 20 pas. Fond exact stationnaire,
   pression dynamique ρga constante : η'=v=0. Erreur de hauteur <=4 ulp du repos,
   critère d'arrondi ; témoin ancien premier pas ±dt Ua/dx, défaut >100 ulp.
2. Flux de bande signé contre intégrale indépendante de courant constant, y
   compris bande coupée par le fond solide ; bilan discret global = différence
   des deux flux, tolérance d'arrondi f32. Flux calculés avant modification de η'.
3. Refus d'élévation incohérente sur une frontière, état publié inchangé.
4. Un pas avec flux de bord non nul : chaque expiration restaure l'état ; reprise
   identique au bit et zéro allocation, via compteur réel de l'intégration.
5. Régressions du workspace release, dont identité du fond nul S253 et harmoniques
   stationnaires. Aucun seuil physique relevé après mesure.


## Défaut reproduit et construction

À la révision de contrat 54e76be, le nouveau test stationnaire échoue au premier
pas : dx=0,25, U=-0,5, a=-0,125, erreur maximale 0,0002501011 m. L'incrément
analytique fautif vaut 0,00025 m ; différence due à l'arrondi f32 de la hauteur.
Le défaut est donc distingué avant modification de la bibliothèque.

ADR-165 sépare la fermeture de la perturbation du flux prescrit. Les faces de
projection restent fermées, mais la bande entre repos et surface totale est
intégrée aux deux extrémités. Les deux flux sont calculés avant l'écriture des
hauteurs, puis participent à la différence conservative. Pas de nouvel état,
pas de dépendance ; API inchangée, contrat de cohérence des élévations étendu
aux deux colonnes u extérieures. Gardes de hauteur au bord avant et après le pas.

## Réception ciblée

Les huit cas stationnaires (deux signes de courant, deux signes de hauteur,
deux mailles) tiennent vingt pas avec **erreur exactement nulle** sur la hauteur
publiée ; vitesses sous la borne d'arrondi. Les intégrales signées passent aussi
quand la bande rencontre le fond solide, contre la longueur exacte en f64.
Ces cas de quadrature isolés ne reçoivent pas une bathymétrie dynamique.

Le bilan global compensé passe à 1e-10 m² du flux net prescrit ; entrées gauche et
droite distinctes, donc le test ne peut passer par simple annulation symétrique.
Élévation incohérente et bord hors domaine refusés sans publication. Fond nul :
aucun nouveau transport imposé ; identité au bit couverte par S253.

```powershell
cargo test --release --offline --locked --manifest-path code/Cargo.toml -p water-core s270 -- --nocapture
cargo test --release --offline --locked --manifest-path code/Cargo.toml
```

## Limites

Reçu stationnaire traversant et bilan de la bande, pas houle incidente progressive
complète. Le bord de v reste fermé, sa transparence dépend de l'éponge et du cas.
Ni front de courant, ni advection ouverte générale de η', ni fond solide traversant,
ni transduction W, ni B4 global, ni budget mural reçus. La mesure S269 à fond nul
n'est pas extrapolée à un fond non nul. Prochain cas utile : onde progressive de
profondeur finie, fond incompressible à W(b)=0, référence et témoins déclarés avant
mesure ; comparer les perturbations induites sans confondre le fond analytique
avec une onde propagée par le solveur total.


## Verdict final

À la révision de construction 453b117 : **478 tests réussis, 18 ignorés, 0 échec**
(workspace release hors ligne verrouillé : cœur 366, intégrations 17, harnais 95).
Les quatre essais unitaires S270 et l'essai d'intégration passent. **638 expirations**
avec flux de fond non nul : champs et hauteur restaurés, reprise identique au bit,
zéro allocation. Les tests ignorés conservent leur statut de bancs explicites.

Critères du lot tenus : le pas mobile consomme désormais le flux de bande prescrit
sans créer le défaut stationnaire reproduit. Identité S253 au fond nul et témoin
harmonique stationnaire conservés par la suite. Aucun reçu supplémentaire sur la
houle progressive, le coût ou une seconde plateforme.
