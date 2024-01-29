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
#include <optional>
#include <string_view>

#include <questdb/conf_str.h>


class parse_err {
public:
    parse_err(questdb_conf_str_parse_err* impl) : _impl(impl) {}

    std::string_view msg() const noexcept {
        return { _impl->msg, _impl->msg_len };
    }

    size_t pos() const noexcept {
        return _impl->pos;
    }

    ~parse_err() noexcept {
        questdb_conf_str_parse_err_free(_impl);
    }

private:
    questdb_conf_str_parse_err* _impl;
};


class conf_str {
public:
    static conf_str parse(std::string_view str) {
        questdb_conf_str_parse_err* err = nullptr;
        auto res = ::questdb_conf_str_parse(str.data(), str.size(), &err);
        if (res != nullptr) {
            return conf_str{res};
        } else {
            throw parse_err(err);
        }
    }

    std::string_view service() const noexcept {
        size_t service_len = 0;
        auto str = ::questdb_conf_str_service(_impl, &service_len);
        return { str, service_len };
    }

    std::optional<std::string_view> get(std::string_view key) const noexcept {
        size_t val_len = 0;
        auto str = ::questdb_conf_str_get(_impl, key.data(), key.size(), &val_len);
        if (str != nullptr) {
            return { { str, val_len } };
        }
        return {};
    }

    ~conf_str() noexcept {
        ::questdb_conf_str_free(_impl);
    }

private:
    conf_str(::questdb_conf_str* impl) : _impl(impl) {}
    ::questdb_conf_str* _impl;
};


static void t1() {
    const auto c1 = conf_str::parse("http");
    assert(c1.service() == "http");
}

static void t2() {
    const auto c1 = conf_str::parse("http::host=localhost;port=9000;");
    assert(c1.service() == "http");
    assert(c1.get("host") == "localhost");
    assert(c1.get("port") == "9000");
}

static void t3() {
    try {
        const auto c1 = conf_str::parse("http;port=9000");
        abort();
    } catch (const parse_err& e) {
        assert(e.msg() == "bad separator, expected ':' got ';' at position 4");
        assert(e.pos() == 4);
    }
}

int main() {
    std::cerr << "Running tests" << std::endl;

    std::cerr << "t1" << std::endl;
    t1();

    std::cerr << "t2" << std::endl;
    t2();

    std::cerr << "t3" << std::endl;
    t3();

    std::cerr << "All tests passed" << std::endl;
    return 0;
}