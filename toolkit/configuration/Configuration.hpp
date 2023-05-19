//
// Created by a.pesterev on 18.04.2023.
//

#ifndef PACKDESIGNER_CONFIGURATION_HPP
#define PACKDESIGNER_CONFIGURATION_HPP

template<typename _size_type, typename _cell_size_type>
class Configuration {
public:

    Configuration() = delete;

    explicit Configuration(_size_type sx, _size_type sy, _cell_size_type minSx, _cell_size_type minSy,
                           _cell_size_type maxSx,
                           _cell_size_type maxSy) : sx(sx), sy(sy), min_sx(minSx), min_sy(minSy), max_sx(maxSx),
                                                    max_sy(maxSy) {}

private:

    _size_type sx;
    _size_type sy;

    _cell_size_type min_sx;
    _cell_size_type min_sy;

    _cell_size_type max_sx;
    _cell_size_type max_sy;

public:

    [[nodiscard]] _size_type getSx() const {
        return sx;
    }

    [[nodiscard]] _size_type getSy() const {
        return sy;
    }

    [[nodiscard]] _cell_size_type getMinSx() const {
        return min_sx;
    }

    [[nodiscard]] _cell_size_type getMinSy() const {
        return min_sy;
    }

    [[nodiscard]] _cell_size_type getMaxSx() const {
        return max_sx;
    }

    [[nodiscard]] _cell_size_type getMaxSy() const {
        return max_sy;
    }

};


Configuration<GridSizeType, CellSizeType> read_json(const std::string &_json_file_path) {
    if (!std::filesystem::exists(_json_file_path) || !std::filesystem::is_regular_file(_json_file_path)) {
        std::cerr << "Passed configuration file does not exist or cannot be found!\n";
        exit(-2);
    }
    Json::Reader _reader;
    Json::Value _root;
    std::ifstream _json_f(_json_file_path);
    if (!_json_f.is_open()) {
        std::cerr << "Could not open root configuration file, given by path = " + _json_file_path << '\n';
        throw std::runtime_error("Could not read json file, given by path = " + _json_file_path);
    }
    const auto could_parse = _reader.parse(_json_f, _root);
    if (_json_f.is_open()) { _json_f.close(); }
    if (!could_parse) {
        std::cerr << _reader.getFormattedErrorMessages();
        std::cerr << "Could not read json file, given by path = " + _json_file_path << '\n';
        throw std::runtime_error("Could not read json file, given by path = " + _json_file_path);
    }
    const auto sx = _root.get("sx", Json::Value(Json::nullValue));
    const auto sy = _root.get("sy", Json::Value(Json::nullValue));
    const auto min_sx = _root.get("min_sx", Json::Value(Json::nullValue));
    const auto min_sy = _root.get("min_sy", Json::Value(Json::nullValue));
    const auto max_sx = _root.get("max_sx", Json::Value(Json::nullValue));
    const auto max_sy = _root.get("max_sy", Json::Value(Json::nullValue));
    if (
            sx.type() != Json::nullValue && sy.type() != Json::nullValue &&
            min_sx.type() != Json::nullValue && min_sy.type() != Json::nullValue &&
            max_sx.type() != Json::nullValue && max_sy.type() != Json::nullValue
            ) {
        return Configuration<GridSizeType, CellSizeType>(
                sx.as<GridSizeType>(),
                sy.as<GridSizeType>(),
                min_sx.as<CellSizeType>(),
                min_sy.as<CellSizeType>(),
                max_sx.as<CellSizeType>(),
                max_sy.as<CellSizeType>()
        );
    }
    std::cerr << "Type mismatch!\n";
    throw std::runtime_error("Undetermined types in root configuration!");
}

#endif //PACKDESIGNER_CONFIGURATION_HPP
