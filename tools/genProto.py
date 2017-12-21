#!/usr/bin/env python3


import os
from os import path
import re
import utils


DIR = path.realpath(path.dirname(utils.getExecPath()))
DIR_PROJ = path.realpath(path.join(DIR, '..'))
DIR_IN = path.join(DIR_PROJ, 'proto')
DIR_OUT = path.join(DIR_PROJ, 'pb')
srcList = []
for dirpath, dirnames, filenames in os.walk(DIR_IN):
    filenames = filter(lambda x: re.match(".+\.proto$", x), filenames)
    for filename in filenames:
        filePath = path.relpath(path.join(dirpath, filename), DIR_IN)
        srcList.append(filePath)
src = ' '.join(srcList)
cmd = 'protoc --rust_out {} -I{} {}'.format(DIR_OUT, DIR_IN, src)
utils.runShell(cmd, cwd=DIR, wait=True, printOutput=True)
