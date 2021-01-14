# ray tracing on one weekend

### Usage
```bash
cargo run --release > image.ppm; convert image.ppm image.png
```

### About

Написал простой
[рейтрейсер](https://ru.wikipedia.org/wiki/%D0%A2%D1%80%D0%B0%D1%81%D1%81%D0%B8%D1%80%D0%BE%D0%B2%D0%BA%D0%B0_%D0%BB%D1%83%D1%87%D0%B5%D0%B9)
на rust по туториалу
[Ray Tracing in One Weekend v3.2.3](https://raytracing.github.io/books/RayTracingInOneWeekend.html).
Это первый из трёх туториалов в [серии](https://raytracing.github.io).

Преследовал я 2 цели, первую достиг, вторую не очень:
1. Прежде всего я изучал rust. Туториал написан на c++, так что по сути я переписывал плюсы на rust.
   Эту цель я достиг, так как больше узнал про модули и dynamic dispatch.
2. Заодно с изучением языка я хотел узнать как работает трассировка лучей. Не скажу, что туториал
   хорошо объясняет материал, хотелось бы больше объяснений на пальцах. Поэтому не уверен, что буду
   проходить оставшиеся 2 туториала из серии.

Зато получилась красивая картинка:
![13.1 Final Render](images/13.1.png?raw=true "13.1 Final Render")

### Notes
To convert all `.ppm` images to `.png`:
```bash
find . -iname "*.ppm" -exec sh -c 'convert {} $(basename {} .ppm).png' \;
```
