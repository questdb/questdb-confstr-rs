/*******************************************************************************
 *     ___                  _   ____  ____
 *    / _ \ _   _  ___  ___| |_|  _ \| __ )
 *   | | | | | | |/ _ \/ __| __| | | |  _ \
 *   | |_| | |_| |  __/\__ \ |_| |_| | |_) |
 *    \__\_\\__,_|\___||___/\__|____/|____/
 *
 *  Copyright (c) 2014-2019 Appsicle
 *  Copyright (c)  2019-2025 QuestDB
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 ******************************************************************************/

#include <iostream>
#include <unordered_map>
#include <vector>

#include <questdb/conf_str.hpp>

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.hpp"

using namespace questdb::conf_str;

TEST_CASE("basic no params")
{
    const auto c1 = conf_str::parse("http");
    CHECK(c1.service() == "http");
    CHECK(c1.get("host") == std::nullopt);
}

TEST_CASE("basic with params")
{
    const auto c1 = conf_str::parse("http::host=localhost;port=9000;");
    CHECK(c1.service() == "http");
    CHECK(c1.get("host") == "localhost");
    CHECK(c1.get("port") == "9000");
}

TEST_CASE("parse error")
{
    auto str = "http;port=9000";
    REQUIRE_THROWS_AS(conf_str::parse(str), parse_err);
    try {
        conf_str::parse(str);
    } catch (const parse_err& e) {
        CHECK(e.msg() == "bad separator, expected ':' got ';' at position 4");
        CHECK(e.pos() == 4);
    }
}

TEST_CASE("iter params") {
    const auto c1 = conf_str::parse("http::host=localhost;port=9000;");
    std::vector<std::pair<std::string_view, std::string_view>> params;
    for (auto it = c1.begin(); it != c1.end(); ++it) {
        params.emplace_back(it.key(), it.value());
    }
    CHECK(params.size() == 2);
    CHECK(params[0].first == "host");
    CHECK(params[0].second == "localhost");
    CHECK(params[1].first == "port");
    CHECK(params[1].second == "9000");
}

TEST_CASE("duplicate keys via iterator") {
    const auto c1 = conf_str::parse("http::addr=host1:9000;addr=host2:9001;");
    std::vector<std::pair<std::string_view, std::string_view>> params;
    for (auto it = c1.begin(); it != c1.end(); ++it) {
        params.emplace_back(it.key(), it.value());
    }
    CHECK(params.size() == 2);
    CHECK(params[0].first == "addr");
    CHECK(params[0].second == "host1:9000");
    CHECK(params[1].first == "addr");
    CHECK(params[1].second == "host2:9001");
}

TEST_CASE("get_all") {
    const auto c1 = conf_str::parse("http::addr=host1:9000;addr=host2:9001;port=9000;");
    auto addrs = c1.get_all("addr");
    CHECK(addrs.size() == 2);
    CHECK(addrs[0] == "host1:9000");
    CHECK(addrs[1] == "host2:9001");

    auto ports = c1.get_all("port");
    CHECK(ports.size() == 1);
    CHECK(ports[0] == "9000");

    auto missing = c1.get_all("nonexistent");
    CHECK(missing.empty());

    // get() still returns the last value
    CHECK(c1.get("addr") == "host2:9001");
}

