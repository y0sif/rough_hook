/****************************************************************************
** Meta object code from reading C++ file 'roundrobintournament.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/lib/src/roundrobintournament.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'roundrobintournament.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_RoundRobinTournament_t {
    QByteArrayData data[1];
    char stringdata0[21];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_RoundRobinTournament_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_RoundRobinTournament_t qt_meta_stringdata_RoundRobinTournament = {
    {
QT_MOC_LITERAL(0, 0, 20) // "RoundRobinTournament"

    },
    "RoundRobinTournament"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_RoundRobinTournament[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       0,    0, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

       0        // eod
};

void RoundRobinTournament::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    (void)_o;
    (void)_id;
    (void)_c;
    (void)_a;
}

QT_INIT_METAOBJECT const QMetaObject RoundRobinTournament::staticMetaObject = { {
    QMetaObject::SuperData::link<Tournament::staticMetaObject>(),
    qt_meta_stringdata_RoundRobinTournament.data,
    qt_meta_data_RoundRobinTournament,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *RoundRobinTournament::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *RoundRobinTournament::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_RoundRobinTournament.stringdata0))
        return static_cast<void*>(this);
    return Tournament::qt_metacast(_clname);
}

int RoundRobinTournament::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = Tournament::qt_metacall(_c, _id, _a);
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
