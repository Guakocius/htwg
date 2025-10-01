#include <iostream>
#include <array>

const int s3 = 3;
 std::array<int, s3> s0 = {0,0,0}, s1 = {1,2,3}, s2 = {3,2,1};

int count = 0;
int t3 = s3;
int t7 = t3 + s0[0];

int loop(int count);

int end(int count) {
    return count;
}

int loop(int count) {
    for (int i = 0; i < s0.size(); i++) {
        count += 3;
        if (i + 1 == t7) {
            count++;
            end(count);
        }
        int t1 = s1[i];
        count++;
        int t2 = s2[i];
        count++;
        int t3 = 0;
        count += 3;
        if (t1 < t2)  {count++;t3 =  1;}
        count += 3;
        if (t3 != 0) {
            t2 = s0[i];
            count++;
        } else {
            count++;
            t1 = s0[i];
            count++;
            count += 4;
        }
    }
    return count +=3;
}



int main() {
   
    count +=3;
    count +=3;
    std::cout << loop(count) << std::endl;
    return 0;

}


