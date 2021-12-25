# Homework 02: Dáma

Vytvořte struktury a funkce pro hru dáma. K nim připravte dokumentaci a testy. 

Implementujte:
- strukturu Board, která má dvojrozměrné pole obsahující šachovnici. 
- strukturu Game, která má board, a hráče, který je právě na tahu. 
- fuknci move(příjímá hru, pozici na které je figurka, a cílovou pozici), provede kontrolu barvy figurky v návaznosti na tom, kdo je právě na tahu. Zkontroluje, zda je možné figurkou daného typu provést tah. Pokud jsou nějaké soupeřovy figurky k vyhození, tak budou vyhozeny. Pokud se figurka dostala na druhou stranu, tak povýšena na dámu.
- funkci get_available_moves(příjímá hru a pozici figurky nehledě na aktuálního hráče) vracející vektor možných polí na která se dá pohnout figurkou.
- funkci check_end_game_conditions, která příjímá hru a vrací, jestli hra skončila (monad s vítězem) nebo ne.
