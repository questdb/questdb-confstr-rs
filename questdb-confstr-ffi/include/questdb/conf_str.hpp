#pragma once

#include "conf_str.h"

#include <optional>
#include <string_view>

namespace questdb::conf_str
{

class parse_err
{
public:
    parse_err(questdb_conf_str_parse_err* impl) : _impl(impl) {}

    std::string_view msg() const noexcept
    {
        return { _impl->msg, _impl->msg_len };
    }

    size_t pos() const noexcept
    {
        return _impl->pos;
    }

    ~parse_err() noexcept
    {
        questdb_conf_str_parse_err_free(_impl);
    }

private:
    questdb_conf_str_parse_err* _impl;
};

class pair_iter
{
public:
    pair_iter(const pair_iter&) = delete;

    bool operator==(const pair_iter& other) const noexcept
    {
        return _impl == other._impl;
    }

    bool operator!=(const pair_iter& other) const noexcept
    {
        return !(*this == other);
    }

    void operator++() noexcept
    {
        const char* key = nullptr;
        size_t key_len = 0;
        const char* val = nullptr;
        size_t val_len = 0;
        const bool has = ::questdb_conf_str_iter_next(
            _impl,
            &key,
            &key_len,
            &val,
            &val_len);
        if (has)
        {
            _key = { key, key_len };
            _val = { val, val_len };
        }
        else
        {
            _key = {};
            _val = {};
            ::questdb_conf_str_iter_free(_impl);
            _impl = nullptr;
        }
    }

    ~pair_iter() noexcept
    {
        if (_impl != nullptr)
        {
            ::questdb_conf_str_iter_free(_impl);
        }
    }

    std::string_view key() const noexcept
    {
        return _key;
    }

    std::string_view value() const noexcept
    {
        return _val;
    }
private:
    friend class conf_str;
    pair_iter(::questdb_conf_str_iter* impl) : _impl(impl)
    {
        if (_impl != nullptr)
        {
            ++(*this);
        }
    }
    ::questdb_conf_str_iter* _impl;
    std::string_view _key;
    std::string_view _val;
};

class conf_str
{
public:
    static conf_str parse(std::string_view str)
    {
        questdb_conf_str_parse_err* err = nullptr;
        auto res = ::questdb_conf_str_parse(str.data(), str.size(), &err);
        if (res != nullptr)
        {
            return conf_str{res};
        }
        else
        {
            throw parse_err(err);
        }
    }

    std::string_view service() const noexcept
    {
        size_t service_len = 0;
        auto str = ::questdb_conf_str_service(_impl, &service_len);
        return { str, service_len };
    }

    std::optional<std::string_view> get(std::string_view key) const noexcept
    {
        size_t val_len = 0;
        auto str = ::questdb_conf_str_get(_impl, key.data(), key.size(), &val_len);
        if (str != nullptr)
        {
            return { { str, val_len } };
        }
        return {};
    }

    pair_iter begin() const noexcept
    {
        auto iter = ::questdb_conf_str_iter_pairs(_impl);
        return { iter };
    }

    pair_iter end() const noexcept
    {
        return pair_iter{ nullptr };
    }

    ~conf_str() noexcept
    {
        ::questdb_conf_str_free(_impl);
    }

private:
    conf_str(::questdb_conf_str* impl) : _impl(impl) {}
    ::questdb_conf_str* _impl;
};

}
