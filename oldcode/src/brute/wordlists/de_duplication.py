#!/bin/env python3

str_list = []
with open('all.txt', 'r') as fp:
    for i in fp.readlines():
        i = i.strip()
        if len(i) > 0 and i not in str_list:
            str_list.append(i)

with open('de_all.txt', 'w+') as fp:
    for s in str_list:
        fp.writelines(s)
        fp.writelines('\n')
