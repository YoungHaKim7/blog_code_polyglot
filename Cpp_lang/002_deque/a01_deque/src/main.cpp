// https://stackoverflow.com/questions/21117636/how-to-implement-a-least-frequently-used-lfu-cache?rq=3
// Source - https://stackoverflow.com/a/54516986
// Posted by Andrushenko Alexander, modified by community. See post 'Timeline'
// for change history Retrieved 2026-02-01, License - CC BY-SA 4.0

#include <cassert>
#include <iostream>
#include <list>
#include <print>
#include <string>
#include <unordered_map>

typedef unsigned uint;

template <typename K, typename V = K> struct Entry {
    K key;
    V value;
};

template <typename K, typename V = K> class LFUCache {

    typedef typename std::list<Entry<K, V>> ElementList;
    typedef typename std::list<std::pair<uint, ElementList>> FrequencyList;

  private:
    std::unordered_map<K, std::pair<typename FrequencyList::iterator,
                                    typename ElementList::iterator>>
        cacheMap;
    FrequencyList elements;
    uint maxSize;
    uint curSize;

    void incrementFrequency(std::pair<typename FrequencyList::iterator,
                                      typename ElementList::iterator>
                                p) {
        if (p.first == prev(elements.end())) {
            // frequency list contains single list with some frequency, create
            // new list with incremented frequency (p.first->first + 1)
            elements.push_back(
                {p.first->first + 1, {{p.second->key, p.second->value}}});
            // erase and insert the key with new iterator pair
            cacheMap[p.second->key] = {prev(elements.end()),
                                       prev(elements.end())->second.begin()};
        } else {
            // there exist element(s) with higher frequency
            auto pos = next(p.first);
            if (p.first->first + 1 == pos->first)
                // same frequency in the next list, add the element in the begin
                pos->second.push_front({p.second->key, p.second->value});
            else
                // insert new list before next list
                pos =
                    elements.insert(pos, {p.first->first + 1,
                                          {{p.second->key, p.second->value}}});
            // update cachMap iterators
            cacheMap[p.second->key] = {pos, pos->second.begin()};
        }
        // if element list with old frequency contained this singe element,
        // erase the list from frequency list
        if (p.first->second.size() == 1)
            elements.erase(p.first);
        else
            // erase only the element with updated frequency from the old list
            p.first->second.erase(p.second);
    }

    void eraseOldElement() {
        if (elements.size() > 0) {
            auto key = prev(elements.begin()->second.end())->key;
            if (elements.begin()->second.size() < 2)
                elements.erase(elements.begin());
            else
                elements.begin()->second.erase(
                    prev(elements.begin()->second.end()));
            cacheMap.erase(key);
            curSize--;
        }
    }

  public:
    LFUCache(uint size) {
        if (size > 0)
            maxSize = size;
        else
            maxSize = 10;
        curSize = 0;
    }
    void set(K key, V value) {
        auto entry = cacheMap.find(key);
        if (entry == cacheMap.end()) {
            if (curSize == maxSize)
                eraseOldElement();
            if (elements.begin() == elements.end()) {
                elements.push_front({1, {{key, value}}});
            } else if (elements.begin()->first == 1) {
                elements.begin()->second.push_front({key, value});
            } else {
                elements.push_front({1, {{key, value}}});
            }
            cacheMap.insert(
                {key, {elements.begin(), elements.begin()->second.begin()}});
            curSize++;
        } else {
            entry->second.second->value = value;
            incrementFrequency(entry->second);
        }
    }

    bool get(K key, V &value) {
        auto entry = cacheMap.find(key);
        if (entry == cacheMap.end())
            return false;
        value = entry->second.second->value;
        incrementFrequency(entry->second);
        return true;
    }
};

int main() {
    // println!
    // print!
    std::println("test c++ 26");
    // print!
    std::print("test c++ 26");
    std::cout << "test c++ 26";

    bool rc;
    int r;
    LFUCache<int> cache(3); // cache of size 3
    cache.set(1, 1);
    cache.set(2, 2);
    cache.set(3, 3);
    cache.set(2, 4);

    rc = cache.get(1, r);

    assert(rc);
    assert(r == 1);
    // evict old element, in this case 3
    cache.set(4, 5);
    rc = cache.get(3, r);
    assert(!rc);
    rc = cache.get(4, r);
    assert(rc);
    assert(r == 5);

    LFUCache<int, std::string> cache2(2);
    cache2.set(1, "one");
    cache2.set(2, "two");
    std::string val;
    rc = cache2.get(1, val);
    if (rc)
        assert(val == "one");
    else
        assert(false);

    cache2.set(3, "three"); // evict 2
    rc = cache2.get(2, val);
    assert(rc == false);
    rc = cache2.get(3, val);
    assert(rc);
    assert(val == "three");
}
