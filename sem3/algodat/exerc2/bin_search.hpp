#include <stdbool.h>
#include <vector>
#ifndef BIN_SEARCH
#define BIN_SEARCH
class BinarySearch {
  public:
    bool insert(int, std::vector<int>) const;
    bool remove(int, std::vector<int>) const;
    int search(int, std::vector<int>) const;
    BinarySearch() {
      std::vector<int> arr;
    }
    ~BinarySearch();

};

#endif // !BIN_SEARCH
