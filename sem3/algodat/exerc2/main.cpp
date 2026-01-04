#include "bin_search.hpp"
#include <iostream>
#include <vector>

int BinarySearch::search(int k, std::vector<int>v) const {
  for (int i = 0; i < v.size(); i++) {
    if (v[i] == k) {
      return k;
    }
  }
  return -1;
}

bool BinarySearch::insert(int k, std::vector<int>v) const {
  for (int i )

}

int main() {
  std::cout << "Hello World" << std::endl;
  return 0;
}
