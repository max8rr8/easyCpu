ADD R4 R2 R3     # Сложить значение в регистре r2 со значением r3 и сохранить результат в регистр r4;
DEC R5 R5        # Декрементировать(уменьшить на 1) значение в регистре r5;
LOAD R3 R2 +1    # Загрузить значение по адресу R2+1 в регистр R3;
MOV R2 R4        # Переместить значение регистра R4 в регистр R2;
BRANCH.EG R5 -4  # Если в R5 содержится значение больше либо равное 0 то прыгнуть на 4 инструкции назад.
