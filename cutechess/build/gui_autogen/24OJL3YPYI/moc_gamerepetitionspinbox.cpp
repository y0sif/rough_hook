/****************************************************************************
** Meta object code from reading C++ file 'gamerepetitionspinbox.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/gamerepetitionspinbox.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'gamerepetitionspinbox.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_GameRepetitionSpinBox_t {
    QByteArrayData data[8];
    char stringdata0[112];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_GameRepetitionSpinBox_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_GameRepetitionSpinBox_t qt_meta_stringdata_GameRepetitionSpinBox = {
    {
QT_MOC_LITERAL(0, 0, 21), // "GameRepetitionSpinBox"
QT_MOC_LITERAL(1, 22, 20), // "setGamesPerEncounter"
QT_MOC_LITERAL(2, 43, 0), // ""
QT_MOC_LITERAL(3, 44, 17), // "gamesPerEncounter"
QT_MOC_LITERAL(4, 62, 9), // "setRounds"
QT_MOC_LITERAL(5, 72, 6), // "rounds"
QT_MOC_LITERAL(6, 79, 17), // "setTournamentType"
QT_MOC_LITERAL(7, 97, 14) // "tournamentType"

    },
    "GameRepetitionSpinBox\0setGamesPerEncounter\0"
    "\0gamesPerEncounter\0setRounds\0rounds\0"
    "setTournamentType\0tournamentType"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_GameRepetitionSpinBox[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       3,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    1,   29,    2, 0x0a /* Public */,
       4,    1,   32,    2, 0x0a /* Public */,
       6,    1,   35,    2, 0x0a /* Public */,

 // slots: parameters
    QMetaType::Void, QMetaType::Int,    3,
    QMetaType::Void, QMetaType::Int,    5,
    QMetaType::Void, QMetaType::QString,    7,

       0        // eod
};

void GameRepetitionSpinBox::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<GameRepetitionSpinBox *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->setGamesPerEncounter((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 1: _t->setRounds((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 2: _t->setTournamentType((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        default: ;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject GameRepetitionSpinBox::staticMetaObject = { {
    QMetaObject::SuperData::link<QSpinBox::staticMetaObject>(),
    qt_meta_stringdata_GameRepetitionSpinBox.data,
    qt_meta_data_GameRepetitionSpinBox,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *GameRepetitionSpinBox::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *GameRepetitionSpinBox::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_GameRepetitionSpinBox.stringdata0))
        return static_cast<void*>(this);
    return QSpinBox::qt_metacast(_clname);
}

int GameRepetitionSpinBox::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QSpinBox::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 3)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 3;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 3)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 3;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
