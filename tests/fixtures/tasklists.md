basic
.
- [ ] unchecked item 1
- [ ] unchecked item 2
- [ ] unchecked item 3
- [x] checked item 4
.
<ul class="contains-task-list">
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" /> unchecked item 1</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" /> unchecked item 2</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" /> unchecked item 3</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" checked="" /> checked item 4</li>
</ul>
.

dirty
.
-   [ ] unchecked todo item 1
- [ ]
- [  ] not a todo item 2
- [ x] not a todo item 3
- [x ] not a todo item 4
- [ x ] not a todo item 5
-   [x] todo item 6
.
<ul class="contains-task-list">
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" /> unchecked todo item 1</li>
<li>[ ]</li>
<li>[  ] not a todo item 2</li>
<li>[ x] not a todo item 3</li>
<li>[x ] not a todo item 4</li>
<li>[ x ] not a todo item 5</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" checked="" /> todo item 6</li>
</ul>
.

nested
.
# Test 1

1. foo
   * [ ] nested unchecked item 1
   * not a todo item 2
   * not a todo item 3
   * [x] nested checked item 4
2. bar
3. spam

# Test 2

- foo
  - [ ] nested unchecked item 1
  - [ ] nested unchecked item 2
  - [x] nested checked item 3
  - [X] nested checked item 4


.
<h1>Test 1</h1>
<ol>
<li>foo
<ul class="contains-task-list">
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" /> nested unchecked item 1</li>
<li>not a todo item 2</li>
<li>not a todo item 3</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" checked="" /> nested checked item 4</li>
</ul>
</li>
<li>bar</li>
<li>spam</li>
</ol>
<h1>Test 2</h1>
<ul>
<li>foo
<ul class="contains-task-list">
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" /> nested unchecked item 1</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" /> nested unchecked item 2</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" checked="" /> nested checked item 3</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" checked="" /> nested checked item 4</li>
</ul>
</li>
</ul>
.

ordered list
.
1. [x] checked ordered 1
2. [ ] unchecked ordered 2
3. [x] checked ordered 3
4. [ ] unchecked ordered 4

.
<ol class="contains-task-list">
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" checked="" /> checked ordered 1</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" /> unchecked ordered 2</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" checked="" /> checked ordered 3</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" /> unchecked ordered 4</li>
</ol>
.

Tab after task list item marker
.
+ [x]	item 1
+ [ ]		item 2
.
<ul class="contains-task-list">
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" checked="" />	item 1</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" />		item 2</li>
</ul>
.

Form feed after task list item marker
.
+ [x]item 1
+ [ ]item 2
.
<ul class="contains-task-list">
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" checked="" />item 1</li>
<li class="task-list-item"><input class="task-list-item-checkbox" type="checkbox" />item 2</li>
</ul>
.
