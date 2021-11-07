# Opakování z přednášky

## Obsah
1. Docker pro setup prostředí
2. Postgres
3. SQLX
4. ORM & Diesel

## Vzorová aplikace
Vytvořte jednoduchou databázi pro objednání k lékaři. Nejprve ji navrhněte pomocí ERD diagramu. K jeho vytvoření použijte PlantUML. Následně vytvořte pomocí Dockeru prostředí databáze a následně proveďte implementaci pomocí Diesel nebo SQLX. 

## Bodovaná aplikace
Dle assignment.md.

## Bonusová úloha: Účetní systém
Navrhněte a implementujte databázi pro jednoduchý účetní systém. V systému musíte vést dodavatele a odběratele. Od nich přichází doklady (přijaté). Pokud vystavíte někomu fakturu, jedná se o doklad vydaný. Každý doklad obsahuje základní informace jako je číslo dokladu (unikátní pro vystavující firmu), datum vystavení, datum uskutečnění zdanitelného plnění, u dokladu přijatých datum přijetí, položky faktury, možné zaokrouhlení na dokladu a také výslednou tabulku DPH - sazba DPH, částka DPH a částka bez DPH v dané sazbě. Počítejte s tím, že sazba se může v průběhu času měnit. Výsledkem by měl být UML diagram a implementovaná databáze.
