import os
import shutil
import hashlib
from io import StringIO
import collections
from configparser import ConfigParser
import subprocess
import codecs
from contextlib import contextmanager


def dumpLua(d, f=None, index=0):
    if f is None:
        f = StringIO()

    sep = ""
    tab = " " * 4
    f.write("{\n")
    if isinstance(d, list):
        for v in d:
            f.write(sep)
            if sep == '':
                sep = ',\n'
            f.write(tab * (index + 1))
            if v is None:
                continue
            elif isinstance(v, list) or isinstance(v, collections.OrderedDict):
                dumpLua(v, f, index + 1)
            else:
                if isinstance(v, int):
                    f.write('%d' % v)
                elif isinstance(v, float):
                    f.write('%.8f' % v)
                else:
                    v = v.replace('\r', '')
                    v = v.replace('\n', '\\n')
                    f.write('"%s"' % v)
    elif isinstance(d, collections.OrderedDict):
        for k, v in d.iteritems():
            f.write(sep)
            if sep == '':
                sep = ',\n'
            f.write(tab * (index + 1))
            if isinstance(k, int):
                f.write('[%d]' % k)
            else:
                f.write(k)
            f.write(' = ')
            if v is None:
                f.write('nil')
            elif isinstance(v, list) or isinstance(v, collections.OrderedDict):
                dumpLua(v, f, index + 1)
            else:
                if isinstance(v, int):
                    f.write('%d' % v)
                elif isinstance(v, float):
                    f.write('%.8f' % v)
                else:
                    v = v.replace('\r', '')
                    v = v.replace('\n', '\\n')
                    f.write('"%s"' % v)
    f.write("\n" + tab * index + "}")
    if index == 0:
        f.seek(0)
        buf = f.read()
        f.close()
        return buf


def getConfig(DIR, pyFileName):
    bname = os.path.basename(pyFileName)
    iniName = os.path.splitext(bname)[0] + ".ini"
    cfg = ConfigParser()
    cfg.read(os.path.join(DIR, iniName))
    return cfg


def BFS_Dir(path, dirCallback=None, fileCallback=None):
    queue = []
    ret = []
    queue.append(path)
    while len(queue) > 0:
        tmp = queue.pop(0)
        if(os.path.isdir(tmp)):
            ret.append(tmp)
            for item in os.listdir(tmp):
                queue.append(os.path.join(tmp, item))
            if dirCallback:
                dirCallback(tmp)
        elif(os.path.isfile(tmp)):
            ret.append(tmp)
            if fileCallback:
                fileCallback(tmp)
    return ret


def DFS_Dir(path, dirCallback=None, fileCallback=None):
    stack = []
    ret = []
    stack.append(path)
    while len(stack) > 0:
        tmp = stack.pop(len(stack) - 1)
        if(os.path.isdir(tmp)):
            ret.append(tmp)
            for item in os.listdir(tmp):
                stack.append(os.path.join(tmp, item))
            if dirCallback:
                dirCallback(tmp)
        elif(os.path.isfile(tmp)):
            ret.append(tmp)
            if fileCallback:
                fileCallback(tmp)
    return ret


def getSubDir(ROOT, path):
    if ROOT == path:
        return ""
    return path[len(ROOT) + 1:]


def cleanDir(DIR, ignores=[]):
    lists = os.walk(DIR)
    for root, dirs, files in lists:
        for f in files:
            path = os.path.join(root, f)
            tmp = getSubDir(DIR, path)
            finded = False
            for v in ignores:
                if tmp.startswith(v):
                    finded = True
                    break
            if finded:
                continue
            os.remove(path)
        for d in dirs:
            path = os.path.join(root, d)
            tmp = getSubDir(DIR, path)
            finded = False
            for v in ignores:
                if tmp.startswith(v):
                    finded = True
                    break
            if finded:
                continue
            shutil.rmtree(path)


def mkOutDir(DIR_OUT, CLEANUP_FIRST, ignores=[]):
    if os.path.exists(DIR_OUT):
        if CLEANUP_FIRST:
            cleanDir(DIR_OUT, ignores)
    else:
        os.makedirs(DIR_OUT)


def replaceDir(srcRoot, dstRoot):
    import os
    import shutil
    for srcDir, dirs, files in os.walk(srcRoot):
        dstDir = srcDir.replace(srcRoot, dstRoot, 1)
        if not os.path.exists(dstDir):
            os.makedirs(dstDir)
        for file_ in files:
            srcFile = os.path.join(srcDir, file_)
            dstFile = os.path.join(dstDir, file_)
            if os.path.exists(dstFile):
                os.remove(dstFile)
            shutil.copy2(srcFile, dstDir)


def formatName(s):
    nameList = s.split("_")
    ret = ""
    for v in nameList:
        ret += v[0].upper() + v[1:].lower()
    return ret


def runMain(main, waitWindows=True):
    try:
        main()
    except Exception as e:
        print(e)
        import traceback
        traceback.print_exc()
        raise e


def runShell(cmd, cwd=None, wait=False, output=False, printOutput=False):
    print("[runShell] {}".format(cmd))
    print("[in dir] {}".format(cwd))
    if output or printOutput:
        p = subprocess.Popen(cmd, shell=True, cwd=cwd, stdout=subprocess.PIPE,
                             stderr=subprocess.STDOUT)
    else:
        p = subprocess.Popen(cmd, shell=True, cwd=cwd)
    if wait:
        import sys
        temp, err = p.communicate()
        if printOutput:
            sys.stdout.write(temp.decode(sys.stdout.encoding))
        if p.returncode:
            if output:
                raise subprocess.CalledProcessError(returncode=p.returncode, cmd=cmd, output=temp)
            else:
                raise subprocess.CalledProcessError(returncode=p.returncode, cmd=cmd)
        if output:
            return (p.returncode, temp)
        return p.returncode
    return 0


def system(cmd, cwd=None):
    cwd = cwd or os.curdir
    cur = os.curdir
    os.chdir(cwd)
    os.system(cmd)
    os.chdir(cur)


@contextmanager
def pushd(newDir):
    previousDir = os.getcwd()
    os.chdir(newDir)
    yield
    os.chdir(previousDir)


def backUpFile(backUp, filename):
    for v in backUp:
        if v["filename"] == filename:
            return
    f = codecs.open(filename, "r", "utf-8")
    lines = f.readlines()
    f.close()
    d = collections.OrderedDict()
    d["filename"] = filename
    d["lines"] = lines
    backUp.append(d)


def modifyFile(backUp, filename, func):
    alreadyIn = False
    for v in backUp:
        if v["filename"] == filename:
            alreadyIn = True
            break
    f = codecs.open(filename, "r", "utf-8")
    lines = f.readlines()
    f.close()
    if not alreadyIn:
        d = collections.OrderedDict()
        d["filename"] = filename
        d["lines"] = lines
        backUp.append(d)
    f = codecs.open(filename, "w", "utf-8")
    for line in lines:
        func(f, line)
    f.close()


def restoreFile(backUp):
    for v in backUp:
        filename = v["filename"]
        lines = v["lines"]
        f = codecs.open(filename, "w", "utf-8")
        for line in lines:
            f.write(line)
        f.close()


def getExecPath():
    import sys
    if hasattr(sys, "frozen"):
        return os.path.realpath(sys.executable)
    else:
        return os.path.realpath(sys.argv[0])


def md5file(fname):
    hash = hashlib.md5()
    with open(fname, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            hash.update(chunk)
    return hash.hexdigest()


def request(url):
    import requests
    return requests.get(url)


def isWindows():
    import platform
    s = platform.system()
    return s == "Windows" or s.startswith("CYGWIN") or s.startswith("MSYS")
