#pragma once

#include "conf_str.h"

#include <optional>
#include <string_view>


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

    ~conf_str() noexcept
    {
        ::questdb_conf_str_free(_impl);
    }

private:
    conf_str(::questdb_conf_str* impl) : _impl(impl) {}
    ::questdb_conf_str* _impl;
};
