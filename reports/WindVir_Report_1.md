# WindVir - Отчёт 1

WindVir - утилита для симуляции распостранения вируса по воздуху в городской среде. Программа призвана для прогнозирования потенциально опасных райнов в случае распостранения угрозы воздушным путём.

## Технологический стек
Для проекта мы выбрали следующий технологический стек.

* **Rust** - Язык программирования выского уровня, является основным в нашей программе. Отличается хорошей скоростью работы и безопаностью.
* **С** - Язык программирования который знают все в нашей команде, поэтому на нём написаны некоторые наши функции внутри кода RUST для большего вовлечения всех участников команды.
* **OpenGL** - Графическая спецификация, стандарты которой используются для вывода карты города преобразованной из geojson
* **CUDA** - программа-аппаратная архитектура для паралельных вычислений.

## Какой алгоритм создания выпуклых фигур мы выбрали
В качестве основы мы выбрали алгоритм Грэхмана.
Алгоритм Грэхема — это алгоритм для построения выпуклой оболочки множества точек на плоскости.

### Краткое описание работы алгоритма

1. **Нахождение стартовой точки**: Сначала из всех точек выбирается та, которая имеет наименьшую x-координату (если таких точек несколько, выбирается та, у которой y-координата меньше). Эта точка будет начальной точкой.

2. **Сортировка точек**: Все оставшиеся точки сортируются по возрастанию угла, который они образуют со стартовой точкой и осью абсцисс. Если углы равны, то точка, ближайшая к начальной, рассматривается первой.

3. **Построение оболочки**: Начиная со стартовой точки, алгоритм проходит по отсортированному списку точек и добавляет каждую точку в оболочку. Если в какой-то момент обнаруживается, что последняя добавленная точка образует правый поворот с предыдущими двумя точками оболочки, то предпоследняя точка удаляется из оболочки. Этот шаг повторяется до тех пор, пока не будет гарантировано, что добавление новой точки сохраняет выпуклость оболочки.

4. **Завершение**: Как только все точки будут пройдены, алгоритм завершается, и полученная последовательность точек представляет собой вершины выпуклой оболочки.

### Почему мы выбрали именно этот алгоритм

Алгоритм Грэхема имеет несколько преимуществ по сравнению с другими алгоритмами построения выпуклой оболочки, такими как алгоритм Джарвиса или алгоритм QuickHull:

1. **Эффективность по времени**: Алгоритм Грэхема имеет временную сложность $n*log(n)$, что делает его одним из самых быстрых алгоритмов для нахождения выпуклой оболочки для объектов с небольшим количеством вершин. Например, алгоритм Джарвиса имеет сложность $nh$, где $h$ — количество вершин в выпуклой оболочке, что может быть неэффективно для наборов данных с малым числом вершин в оболочке.

2. **Простота реализации**: Несмотря на то, что алгоритм требует предварительной сортировки точек, он остаётся относительно простым для понимания и реализации. Это делает его идеальным выбором для нашего проекта учитывая небольшие сроки.

3. **Устойчивость к коллинеарным точкам**: Алгоритм легко адаптируется для работы с коллинеарными точками , позволяя либо включать их всех в оболочку, либо выбирать только крайние точки. Это универсальность делает алгоритм более безпроблемным в использовании.

4. **Подходит для обработки большого количества зданий**: Благодаря своей эффективности и скорости, алгоритм хорошо подходит для работы с большим количеством небольших зданий, что актуально для обработки местности больших размеров.

5. **Минимальная зависимость от формы входных данных**: В отличие от некоторых других алгоритмов, эффективность алгоритма Грэхема не сильно зависит от распределения или формы входного набора точек.

### Подводные камни с которыми мы можем столкнутся

1. **Чувствительность к точности вычислений**: Алгоритм полагается на вычисление углов и определение направления поворота между точками. Это может привести к проблемам в ситуациях, когда работа ведется с вещественными числами и точность вычислений ограничена, особенно при наличии очень близко расположенных друг к другу точек или при обработке очень больших наборов данных.

2. **Неоптимальность при обработке больших количеств точек вне оболочки**: Хотя сложность алгоритма $n*log(n)$ является достаточно эффективной, в ситуациях, когда большинство из *n* точек находятся вне искомой выпуклой оболочки, алгоритм может оказаться менее эффективным по сравнению с некоторыми другими методами, например, с алгоритмом "QuickHull", который потенциально может лучше справляться с такими наборами данных.

3. **Требования к предварительной сортировке**: Алгоритм требует сортировки точек по углу, что добавляет дополнительный шаг и может увеличивать общее время выполнения, особенно для больших наборов данных. Хотя сама сортировка и имеет логарифмическую сложность, она может быть ресурсоёмкой на практике.

4. **Ограничение на двумерные данные**: Алгоритм изначально разработан для работы с точками на двумерной плоскости. Хотя существуют его модификации для трехмерного пространства, для более высоких измерений алгоритм может не подходить или требовать значительной адаптации.

## Текущее состояние и планы на ближайшее время
На текущий момент нами подготовлен тестовый geojson файл с картой метности города Петровск. Наша программа уже умеет парсить geojson и выводить их на экран с помощью OpenGL, реализовано управление камерой, а так же дополнительные настройки повышающие комфорт пользователя. Эти настройки задаются в .json файле внутри программы.

Следующим этапом мы планируем внедрение алгоритма Грэхема в нашу программу для оптимизации объектов на местности. Это необходимо для будущего просчёта симуляции ветра с помощью CUDA.