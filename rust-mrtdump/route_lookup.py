from pybgpkit_parser import Parser

from route_table import RouteTable

if __name__ == '__main__':

    rt = RouteTable()

    parser = Parser(url="rib.20230626.0400.bz2")
    count = 0
    for element in parser:
        prefix, origin_as = element['prefix'], element['origin_asns']
        prefix, length = prefix.split("/")
        origin_as = origin_as[0]
        length = int(length)
        origin_as = int(origin_as)
        if length != 0 :
            rt.add(prefix, length, origin_as)
            count += 1
            if count % 10000 == 0:
                print(f"{count} entries: prefix: {prefix}/{length}")

    print(rt.lookup('123.252.240.140'))
