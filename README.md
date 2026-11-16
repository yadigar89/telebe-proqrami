# Tələbə İdarəetmə Proqramı

Rust dilində yazılmış sadə terminal proqramı. Tələbə siyahısını saxlayır, əlavə edir, axtarır, silir, sıralayır və fayla yazır.

## İşə salmaq

```
cargo run
```

## Menyu

1. Tələbələrin siyahısı
2. Tələbə əlavə et
3. Fayla saxla
4. Tələbə axtar
5. Tələbə sil
6. Yaşı dəyiş
7. Yaşa görə sırala
8. Yaşdan böyük olanların sayı
9. Çıxış

## Məlumatın saxlanması

Məlumat `telebeler.txt` faylında hər sətirdə bir tələbə olmaqla saxlanılır:

```
Ad, yaş, ixtisas
```

Proqram açılanda fayl oxunur. Fayl yoxdursa, boş siyahı ilə başlayır.

## Testlər

```
cargo test
```

## Öyrənilən mövzular

Rust: struct, enum, Result, fayl oxuma/yazma, modullar, testlər, trait, iter/filter/map.