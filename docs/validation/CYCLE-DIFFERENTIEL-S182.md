# Cycle vivant du consommateur différentiel — S182

S181-1/A50 ; applique ADR-117 sans modifier le contrat de publication.

## Protocole déclaré avant tests

- Comparer toutes les grandeurs différentielles et la source en bits, contrôleur
  de pression contre préparation directe du même journal au même instant. Inclure
  des retours temporels, naissances/extinctions et absence d'actualisation nécessaire.
- Après une mise à jour refusée, retrouver exactement la publication précédente ;
  une vue périmée doit être refusée au nouvel instant sans sortie partielle.
- Admettre une source en dernier (voie incrémentale), puis une source intercalée
  (recalcul complet), et comparer à la voie directe. Recevoir saturation/reprise
  sur un journal élargi, sans confondre dernier champ publié et journal complet.
- Renouveler les impacts via le service vivant, refuser un horizon insuffisant puis
  reprendre ; comparer à une reconstruction directe. Sauvegarder puis restaurer les
  événements et contextes, reconstruire les champs et retrouver dérivées/source.
- Workspace, ciblés release et C02/C18 inchangés. Ces contrôles ne mesurent pas le
  coût, ne reçoivent pas δ3D et ne remplacent pas les références physiques S177–S181.

## Résultats

À compléter en P3. Si les chemins passent sans correction, conserver le code runtime.
