front-matter
.
---
title: "Markdown-it.rs"
---
.
<root srcmap="0:32">
  <front_matter srcmap="0:31">
    content: title: "Markdown-it.rs"
.

strikethrough
.
~~strikethrough~~
.
<root srcmap="0:18">
  <paragraph srcmap="0:17">
    <strikethrough srcmap="0:17">
      marker: ~
      <text srcmap="2:15">
        content: strikethrough
.

table
.
| foo | bar |
| --- | --- |
| baz | bim |
.
<root srcmap="0:42">
  <table srcmap="0:41">
    alignments: ['none', 'none']
    <thead srcmap="0:27">
      <trow srcmap="0:13">
        <tcell srcmap="2:5">
          <text srcmap="2:5">
            content: foo
        <tcell srcmap="8:11">
          <text srcmap="8:11">
            content: bar
    <tbody srcmap="28:41">
      <trow srcmap="28:41">
        <tcell srcmap="30:33">
          <text srcmap="30:33">
            content: baz
        <tcell srcmap="36:39">
          <text srcmap="36:39">
            content: bim
.

linkify
.
https://commonmark.com
.
<root srcmap="0:23">
  <paragraph srcmap="0:22">
    <linkify srcmap="0:22">
      url: https://commonmark.com
      <text_special srcmap="0:22">
        content: https://commonmark.com
        info: autolink
        markup: https://commonmark.com
.

tasklist
.
- [ ] unchecked item
- [x] checked item
.
<root srcmap="0:40">
  <bullet_list class="contains-task-list" srcmap="0:39">
    marker: -
    <list_item class="task-list-item" srcmap="0:20">
      <todo_checkbox>
        checked: False
        disabled: False
      <text srcmap="2:20">
        content:  unchecked item
    <list_item class="task-list-item" srcmap="21:39">
      <todo_checkbox>
        checked: True
        disabled: False
      <text srcmap="23:39">
        content:  checked item
.
