# Homework 03: Wartycoon

Vytvorte jednoduchú hru Wartycoon pre dvoch hráčov. Jedná sa o konzolovú hru, ktorá priebežne vypisuje stav samotnej hry
a umožňuje hráčovi vyberať si typ ťahu. Hra nemá žiadny herný plán.

### Základný prehľad:

Hráč má:

- meno
- budovy (buildings)
- jednotky (units)
- suroviny (resources)

Pre jednoduchosť máme jediný typ budovy:

- základňa (base)

Jednotky môžu byť dvoch druhov:

- bojovníci (warriors)
- lukostrelci (archers)

Suroviny sú tiež dvoch druhov, a to:

- drevo (wood)
- zlato (gold)

Možné typy ťahov sú:

- stavať (build)
- ťažiť (harvest)
- trénovať (train)
- dobývať (conquer)
- ukončiť hru (quit)

### Popis

Základňa slúži na tréning jednotiek. Základňa má kapacitu, ktorú nie je možné prekročiť. V základni je možné trénovať
lukostrelcov aj bojovníkov.

Základňa má vždy fixnú kapacitu, a to 200 ľudí.

Základní môže mať hráč neobmedzený počet. Uvažujeme, že základňa je okamžite vybudovaná a dostupná hneď v ďalšom ťahu.

Bojovníci a lukostrelci môžu byť vyslaní, aby dobyli územie. Pre jednoduchosť budeme uvažovať, že vyslané jednotky sú v
teréne a nevracajú sa späť, ani neumierajú. Vyhráva hráč, ktorého jednotky v teréne prevýšia svojou celkovou "váhou"
jednotky protihráča. Pozor, rátajú sa iba jednotky, ktoré boli naozaj vyslané do terénu.

Lukostrelci majú v teréne vyššiu váhu než bojovníci (viď. nižšie).

Suroviny sa ťažia z okolia a je ich neobmedzený počet. Ťažba každej zvolenej suroviny prináša vždy konštantné množstvo
tejto suroviny (viď. nižšie).

Je možné poslať dobývať ľubovoľný počet jednotiek (max však toľko, koľko má hráč k dispozícii), v rámci jedného ťahu
však môžu byť vyslané iba jednotky rovnakého typu. Trénovať je možné naraz toľko jednotiek, koľko je voľných miest vo
všetkých základniach. Trénovať v jednom ťahu je možné taktiež iba jeden zvolený typ jednotiek (bojovníkov alebo
lukostrelcov, nie však mixovane v tom istom ťahu). Tréning jednotiek je teda obmedzený nielen kapacitou základní, ale aj
množstvom surovín.

Aby teda hráč mohol stavať a trénovať (a následne dobývať), musí pravidelne ťažiť. Súperiace jednotky spolu priamo
nebojujú,
"iba" okupujú územie (pre jednoduchosť predpokladáme, že hráči nevedia o stave jednotiek súpera, a preto musia zvoliť
nejakú stratégiu).

Pri každej akcii treba kontrolovať, či má hráč dostatok surovín, budov či jednotiek.

### Ceny akcií

STAVBA:

- základňa: 100 zlata, 220 dreva

JEDNOTKY:

- bojovník: 10 dreva, 5 zlata
- lukostrelec: 10 zlata

ŤAŽBA:

- drevo: 200 jednotiek
- zlato: 120 jednotiek

VÁHA JEDNOTKY V TERÉNE:

- bojovník: 1.2
- lukostrelec: 1.9

### Požiadavky

Na začiatku musia byť hráči vyzvaní, aby zadali svoje meno.

Taktiež je nutné zadať, koľko kôl budú hráči hrať. Hra sa hrá buď dokým sa neodohrá zadaný počet kôl
alebo na vyžiadanie jedného z hráčov (ťahom 'ukončiť'). V oboch prípadoch sa hra vyhodnotí a vypíše sa, ktorý hráč
vyhral.

V každom kole je najskôr vyzvaný hráč číslo 1, potom hráč číslo 2. Keď obaja hráči odohrajú povolený ťah, vypíše sa stav hry.

Ťah hráč zadáva číselne (číslom 1 až 5). Je preferované dodržať nasledujúce mapovanie akcií:

    1: Build
    2: Harvest
    3: Train
    4: Conquer
    5: Quit game

Táto úloha nebude automaticky testovaná, je však viac než preferované dodržiavať konvencie podľa `cargo clippy` a
`cargo fmt`. 

Nemusíte písať testy ani komentáre, avšak pokiaľ sa vám bude zdať váš kód v niektorých miestach zložitejší, krátky komentár
je vítaný.