import sys
import hashlib

def     find_path( key, x, y ) :
    if x == 3 and y == 3 :
        return ""
    else :
        isLock = hashlib.md5( key ).hexdigest()[0:4]
        locklist = []
        for i in isLock :
            if i < "b" :
                locklist.append(1)
            else :
                locklist.append(0)

        if x == 0 :
            locklist[2] = 1
        elif x == 3 :
            locklist[3] = 1
        if y == 0 :
            locklist[0] = 1
        elif y == 3 :
            locklist[1] = 1

        path = ["NG", "NG", "NG", "NG"]
        add_val_x = [ 0,  0, -1,  1]
        add_val_y = [-1,  1,  0,  0]
        add_str = ["U", "D", "L", "R"]

        for i in range(len(locklist)) :
            if locklist[i] == 0 :
                ans = find_path( key + add_str[i].encode('utf-8'), x + add_val_x[i], y + add_val_y[i] )
                if ans != "NG" :
                    path[i] = ans

        min_path_num = 0
        min_path = "NG"

        for i in range(len(path)) :
            if path[i] != "NG" :
                if min_path == "NG" or min_path_num > len(path[i]) :
                    min_path_num = len(path[i])
                    min_path = add_str[i] + path[i]
        return min_path

if __name__ == "__main__":
    args = sys.argv
    if len(args) != 2 :
        print ( "input key!!" )
    else :
        ret = find_path( args[1].encode('utf-8'), 0, 0 )
        if ret == "NG" :
            print( "This case is impossible." )
        else :
            print( "ans:" + ret )
