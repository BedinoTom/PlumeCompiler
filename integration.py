import os
import subprocess

c_ok=0
c_err=0
def test_directory(dir):
    global c_ok,c_err
    for file in os.listdir(dir):
        data = file.split(".")
        if data[1] == "s":
            print(os.path.join(dir,file))
            result = subprocess.run(["./target/debug/PlumeCompiler.exe",os.path.join(dir,file)])
            if result.returncode==0:
                bin_file=open(os.path.join(dir,data[0]+".bin"))
                buffer=bin_file.read()
                bin_file_attempt=open(os.path.join(".","bin",file+".bin"))
                buffer2=bin_file_attempt.read()
                if buffer==buffer2:
                    c_ok=c_ok+1
                else:
                    print("B1:"+buffer)
                    print("B2:"+buffer2)
                    c_err=c_err+1
            else:
                raise Exception("Error")

try:
    for test in os.listdir("./global_test"):
        test_directory(os.path.join(".","global_test",test))
    print("c_ok:"+str(c_ok))
    print("c_err:"+str(c_err))
except:
    pass