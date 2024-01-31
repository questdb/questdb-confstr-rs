/*******************************************************************************
 *     ___                  _   ____  ____
 *    / _ \ _   _  ___  ___| |_|  _ \| __ )
 *   | | | | | | |/ _ \/ __| __| | | |  _ \
 *   | |_| | |_| |  __/\__ \ |_| |_| | |_) |
 *    \__\_\\__,_|\___||___/\__|____/|____/
 *
 *  Copyright (c) 2014-2019 Appsicle
 *  Copyright (c) 2019-2024 QuestDB
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
    std::unordered_map<std::string, std::string> params;
    for (auto it = c1.begin(); it != c1.end(); ++it) {
        params.emplace(it.key(), it.value());
    }
    CHECK(params.size() == 2);
    CHECK(params["host"] == "localhost");
    CHECK(params["port"] == "9000");
}

